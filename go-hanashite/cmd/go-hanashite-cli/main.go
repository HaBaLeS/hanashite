package main

import (
	"fmt"
	"github.com/habales/hanashite/go/pipeline"
	"html/template"
	"os"
	"os/signal"

	"github.com/gordonklaus/portaudio"
	"github.com/teris-io/cli"
)

//Version is set with LDFLAGS in Makefile
var Version = "No Version Provided"

//Build is set with LDFLAGS in Makefile
var Build string

var tmpl = template.Must(template.New("").Parse(
	`{{. | len}} host APIs: {{range .}}
	Name:                   {{.Name}}
	{{if .DefaultInputDevice}}Default input device:   {{.DefaultInputDevice.Name}}{{end}}
	{{if .DefaultOutputDevice}}Default output device:  {{.DefaultOutputDevice.Name}}{{end}}
	Devices: {{range .Devices}}
		Name:                      {{.Name}}
		MaxInputChannels:          {{.MaxInputChannels}}
		MaxOutputChannels:         {{.MaxOutputChannels}}
		DefaultLowInputLatency:    {{.DefaultLowInputLatency}}
		DefaultLowOutputLatency:   {{.DefaultLowOutputLatency}}
		DefaultHighInputLatency:   {{.DefaultHighInputLatency}}
		DefaultHighOutputLatency:  {{.DefaultHighOutputLatency}}
		DefaultSampleRate:         {{.DefaultSampleRate}}
	{{end}}
{{end}}`,
))

func main() {

	dlist := cli.NewCommand("list", "Enumerate Devices").
		WithAction(listDevicesAction)

	version := cli.NewCommand("version", "Show Version information").WithAction(func(args []string, options map[string]string) int {
		fmt.Printf("話して >> Hanashite: %s\t", Version)
		fmt.Printf("Commit: %s\n", Build)
		return 0
	})

	ppl := cli.NewCommand("pipeline", "Start full pipeline").WithAction(pipelineAction)
	app := cli.New("Commandline Version of go-hanashite").
		WithCommand(dlist).
		WithCommand(version).
		WithCommand(ppl)

	os.Exit(app.Run(os.Args, os.Stdout))

}

func pipelineAction(args []string, options map[string]string) int {
	fmt.Println("Start Pipeline")

	//Handle Ctrl+C
	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, os.Kill)



	//Create processors
	pap := pipeline.NewPortAudioProcessor(60)
	pap.InitAudio()
	defer pap.TerminateAudio()

	uvm := pipeline.NewUVMeter()
	udpproc, err := pipeline.NewUDPProc("davidhausen.de:9876")
	panicOnError(err)

	opus, err  := pipeline.NewOpusCodec(48000, 1)
	panicOnError(err)

	var framecount uint64 = 0
	//Setup Outgoing Pipeline
	sendingPipeline := pipeline.NewPipeline().
		AddProcessor(func(frame *pipeline.AudioFrame) {
			frame.FrameNum = framecount
			framecount++
		}).
		AddProcessor(pap.RecorderFrameProcessor()).
		AddProcessor(uvm.FrameProcessor()).
		AddProcessor(opus.EncodeFrameProcessor()).
		AddProcessor(udpproc.OutgoingFrameProcessor())


	//Set up Incoming pipeline
	receivePipeline := pipeline.NewPipeline().
		AddProcessor(udpproc.IncomingFrameProcessor()).
		AddProcessor(opus.DecodeFrameProcessor()).
		AddProcessor(pap.PlayerFrameProcessor())

	go func(){
		for true {
			sendingPipeline.Process(&pipeline.AudioFrame{})
		}
	}()

	go func(){
		for true {
			receivePipeline.Process(&pipeline.AudioFrame{})
		}
	}()

	//wait for Crtl+C
	for {
		select {
		case <-sig:
			fmt.Println("\nStopping")
			pap.Shutdown()
			return 0
		default:
		}
	}

}

func panicOnError(err error) {
	if err != nil {
		panic(err)
	}
}

func listDevicesAction(args []string, options map[string]string) int {

	pa := pipeline.NewPortAudioProcessor(20)
	pa.InitAudio()
	defer pa.TerminateAudio()

	hs, err := portaudio.HostApis()
	if err != nil {
		panic(err)
	}

	err = tmpl.Execute(os.Stdout, hs)
	if err != nil {
		panic(err)
	}

	return 0
}
