use itertools::Itertools;
use regex::{Regex, RegexBuilder};
use std::thread;
use std::{collections::HashMap, error::Error};
use subprocess::{Popen, PopenConfig, Redirection};

use lazy_static::lazy_static;
lazy_static! {
    static ref RE_WIFI_PROFILE: Regex = RegexBuilder::new(r"All User Profile\s+:\s(?P<profile>.*)$")
        .multi_line(true)
        .build()
        .unwrap();
    static ref RE_WLAN_KEY: Regex = RegexBuilder::new(r"Key Content\s+:\s(?P<ssid>.*)$")
        .multi_line(true)
        .build()
        .unwrap();
    // static ref found_ssids: Arc<Mutex<HashMap<String, String>>> =
    //     Arc::new(Mutex::new(HashMap::new()));
}

fn main() -> Result<(), Box<dyn Error>> {
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

    // let mut found_ssids = HashMap::new();
    let mut found_ssids: HashMap<String, String> = HashMap::new();

    for profile in RE_WIFI_PROFILE.captures_iter(&profiles_stdout) {
        let ssid = profile["profile"].trim().to_string();
        found_ssids.insert(ssid, String::new());
    }

    // println!("{:?}", found_ssids);

    if found_ssids.is_empty() {
        eprintln!("No WiFi profiles found.");
        std::process::exit(1);
    }

    println!("Loading passwords...\n");

    thread::scope(|s| {
        for (ssid, value) in found_ssids.iter_mut() {
            s.spawn(move || {
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

                let mut proc = proc.expect("Failed to create process.");

                let (out, _) = proc.communicate(None).unwrap();
                let ssid_stdout = out.unwrap();

                for wifi in RE_WLAN_KEY.captures_iter(&ssid_stdout) {
                    let password = wifi["ssid"].trim().to_string();
                    *value = password;
                }
            });
        }
    });

    let mut i = 1;
    for (ssid, password) in found_ssids.iter().sorted() {
        println!("\t{i}. {ssid}: \"{password}\"");
        i += 1;
    }
    std::process::exit(0);
}
