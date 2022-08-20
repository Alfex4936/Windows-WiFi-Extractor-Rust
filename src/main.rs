use itertools::Itertools;
use regex::RegexBuilder;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{collections::HashMap, error::Error};
use subprocess::{Popen, PopenConfig, Redirection};

fn main() -> Result<(), Box<dyn Error>> {
    let re_wifi_profile = RegexBuilder::new(r"All User Profile\s+:\s(?P<profile>.*)$")
        .multi_line(true)
        .build()
        .unwrap();
    let re_wlan_key = RegexBuilder::new(r"Key Content\s+:\s(?P<ssid>.*)$")
        .multi_line(true)
        .build()
        .unwrap();

    println!("Loading all WiFis saved on computer...");
    let cmd: Vec<&str> = r#"netsh wlan show profiles"#.split(" ").map(|s| s).collect();

    // let cmds = vec![Exec::shell("netsh wlan show profiles"), Exec::shell("sort")];
    // let proc = subprocess::Pipeline::from_exec_iter(cmds).capture();

    let proc = Popen::create(
        &cmd,
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::None,
            stderr: Redirection::None,
            ..Default::default()
        },
    );

    let mut proc = match proc {
        Ok(proc) => proc,
        Err(e) => {
            panic!("Failed to create process: {:?}", e)
        }
    };

    let (out, _) = proc.communicate(None).unwrap();
    let profiles_stdout = out.unwrap();

    let mut found_ssids = HashMap::new();

    for profile in re_wifi_profile.captures_iter(&profiles_stdout) {
        let ssid = profile["profile"].trim().to_string();
        found_ssids.insert(ssid, String::new());
    }

    // println!("{:?}", found_ssids);

    if found_ssids.is_empty() {
        eprintln!("No WiFi profiles found.");
        std::process::exit(1);
    }

    println!("Loading passwords...");

    for (ssid, value) in found_ssids.iter_mut() {
        // TODO - 멀티 쓰레딩으로 Go언어 GUI 버전처럼 바꾸기

        let cmd = format!("netsh wlan show profile {ssid} key=clear");
        let cmd: Vec<&str> = cmd.as_str().split(" ").map(|s| s).collect();

        let proc = Popen::create(
            &cmd,
            PopenConfig {
                stdout: Redirection::Pipe,
                stdin: Redirection::None,
                stderr: Redirection::None,
                ..Default::default()
            },
        );

        let mut proc = match proc {
            Ok(proc) => proc,
            Err(e) => {
                panic!("Failed to create process: {:?}", e)
            }
        };

        let (out, _) = proc.communicate(None).unwrap();
        let ssid_stdout = out.unwrap();

        for wifi in re_wlan_key.captures_iter(&ssid_stdout) {
            let password = wifi["ssid"].trim().to_string();
            *value = password;
        }
    }

    for (ssid, password) in found_ssids.iter().sorted() {
        println!("Wifi: {}, Password: {}", ssid, password);
    }
    Ok(())
}
