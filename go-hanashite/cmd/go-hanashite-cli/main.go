package main

import (
	"fmt"
	"os"

	"github.com/gordonklaus/portaudio"
	hanashite "github.com/habales/hanashite/go"
	"github.com/teris-io/cli"
)

//Version is set with LDFLAGS in Makefile
var Version = "No Version Provided"

//Build is set with LDFLAGS in Makefile
var Build string

func main() {

	dlist := cli.NewCommand("devicelist", "Enumerate Devices").
		WithShortcut("l").
		WithAction(listDevices)

	version := cli.NewCommand("version", "Show Version information").WithAction(func(args []string, options map[string]string) int {
		fmt.Printf("話して >> Hanashite: %s\t", Version)
		fmt.Printf("Commit: %s\n", Build)
		return 0
	})

	app := cli.New("Commandline Version of go-hanashite").
		WithCommand(dlist).
		WithCommand(version)
	os.Exit(app.Run(os.Args, os.Stdout))

}

func listDevices(args []string, options map[string]string) int {

	hanashite.InitAudio()
	defer hanashite.TerminateAudio()

	i, err := portaudio.HostApis()
	if err != nil {
		panic(err)
	}
	for _, h := range i {
		fmt.Printf("Name: %s\n", h.Name)

	}
	return 0
}
