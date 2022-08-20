package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"regexp"
	"strings"
	"sync"
)

var lock = sync.RWMutex{}

func main() {
	// netsh wlan show profiles
	found_ssids := make(map[string]string)

	profiles_stdout, err := exec.Command("cmd.exe", "/c", "start", "/b", "netsh", "wlan", "show", "profiles").Output()
	if err != nil {
		fmt.Println("An error occured while starting a process")
		os.Exit(1)
	}

	all_pat := regexp.MustCompile(`(?m)All User Profile\s+:\s(.*)$`)
	all_matches := all_pat.FindAllStringSubmatch(string(profiles_stdout), -1) // matches is [][]string

	for _, match := range all_matches {
		wifi := strings.TrimRight(match[1], "\r\n")
		found_ssids[wifi] = ""
	}

	if len(found_ssids) == 0 {
		fmt.Println("No wifi profiles found")
		os.Exit(1)
	}

	profile_pat := regexp.MustCompile(`(?m)Key Content\s+:\s(.*)$`)

	var wg sync.WaitGroup
	wg.Add(len(found_ssids))

	fmt.Printf("Getting passwords...\n")
	for ssid := range found_ssids {
		go func(ssid string) {
			defer wg.Done()
			profile_stdout, _ := exec.Command("cmd.exe", "/c", "start", "/b", "netsh", "wlan", "show", "profile", ssid, "key=clear").Output()
			pw_match := profile_pat.FindStringSubmatch(string(profile_stdout))

			var pw_match_string string
			if len(pw_match) != 0 {
				pw_match_string = pw_match[1]
			}

			stdout_password := strings.TrimRight(pw_match_string, "\r\n")

			lock.Lock()
			defer lock.Unlock()
			found_ssids[ssid] = stdout_password
		}(ssid)
	}

	wg.Wait()

	fmt.Printf("Finished loading!\n\n")

	i := 0
	for s, p := range found_ssids {
		fmt.Printf("\t%d. %s: \"%s\" \n", i, s, p)
		i++
	}

	fmt.Print("\nPress 'Enter' to terminate...")
	_, err = bufio.NewReader(os.Stdin).ReadBytes('\n')
	if err != nil {
		os.Exit(1)
	}
}
