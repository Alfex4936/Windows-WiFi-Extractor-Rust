package main

import (
	"context"
	"fmt"
	"log"
	"os/exec"
	"regexp"
	"strings"
	"sync"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

var lock = sync.RWMutex{}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

func (a *App) LoadWifi() map[string]string {
	// netsh wlan show profiles
	found_ssids := make(map[string]string)

	profiles_stdout, err := exec.Command("cmd.exe", "/c", "start", "/b", "netsh", "wlan", "show", "profiles").Output()
	if err != nil {
		log.Fatal(err)
		return found_ssids
	}

	all_pat := regexp.MustCompile(`(?m)All User Profile\s+:\s(.*)$`)
	all_matches := all_pat.FindAllStringSubmatch(string(profiles_stdout), -1) // matches is [][]string

	for _, match := range all_matches {
		wifi := strings.TrimRight(match[1], "\r\n")
		found_ssids[wifi] = ""
	}

	if len(found_ssids) == 0 {
		return found_ssids
	}

	profile_pat := regexp.MustCompile(`(?m)Key Content\s+:\s(.*)$`)

	var wg sync.WaitGroup
	wg.Add(len(found_ssids))

	for ssid, _ := range found_ssids {
		go func(ssid string) {
			defer wg.Done()
			fmt.Printf("getting passwod of %s...\n", ssid)
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

	fmt.Printf("Finished loading!\n")
	return found_ssids
}
