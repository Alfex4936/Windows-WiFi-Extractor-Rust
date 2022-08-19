package main

import (
	"context"
	"fmt"
	"log"
	"os/exec"
	"regexp"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

func (a *App) LoadWifi() string {
	// netsh wlan show profiles
	profiles_stdout, err := exec.Command("cmd.exe", "/c", "start", "/b", "netsh", "wlan", "show", "profiles").Output()
	if err != nil {
		log.Fatal(err)
		return err.Error()
	}
	// result, _ := iconv.ConvertString(string(out), "euc-kr", "utf-8")

	// fmt.Printf("The date is %s\n", result)

	// found_ssids := make(map[string]string)

	pat := regexp.MustCompile(`All User Profile\s+:\s(.*)$`)
	matches := pat.FindAllStringSubmatch(string(profiles_stdout), -1) // matches is [][]string
	fmt.Printf("%q\n", matches)

	for _, match := range matches {
		fmt.Printf("profile=%s\n", match[1])
		// found_ssids[]
	}

	return string(profiles_stdout)
}
