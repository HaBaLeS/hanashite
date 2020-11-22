package main

import (
	"fmt"
	"github.com/habales/hanashite/go/pipeline"
	"html/template"
	"os"
	"os/signal"

	"github.com/gordonklaus/portaudio"
	hanashite "github.com/habales/hanashite/go"
	"github.com/habales/hanashite/go/serialize"
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

	recorder := cli.NewCommand("record", "Record raw pcm file").WithAction(recordAction).WithArg(cli.NewArg("file", "filename to save into"))
	player := cli.NewCommand("play", "Play raw pcm file").WithAction(playerAction).WithArg(cli.NewArg("file", "filename play from"))
	encode := cli.NewCommand("encode", "Encode raw file to opus").
		WithAction(encodeAction).
		WithArg(cli.NewArg("in", "rawpcm to encode")).
		WithArg(cli.NewArg("out", "file to write opus frames to"))

	decode := cli.NewCommand("decode", "Decode opus frames to raw pcm").
		WithAction(decodeAction).
		WithArg(cli.NewArg("in", "opus to decode")).
		WithArg(cli.NewArg("out", "file to write pcm to"))

	ppl := cli.NewCommand("pipeline", "Start full pipeline").WithAction(pipelineAction)


	app := cli.New("Commandline Version of go-hanashite").
		WithCommand(dlist).
		WithCommand(version).
		WithCommand(recorder).
		WithCommand(encode).
		WithCommand(decode).
		WithCommand(player).
		WithCommand(ppl)

	os.Exit(app.Run(os.Args, os.Stdout))

}

func pipelineAction(args []string, options map[string]string) int {
	fmt.Println("Start Pipeline")

	hanashite.InitAudio()
	defer hanashite.TerminateAudio()

	uvm := pipeline.NewUVMeter()
	udpproc, err := pipeline.NewUDPProc("davidhausen.de:9876")
	panicOnError(err)

	encoder, err  := pipeline.NewOpusEncoder(int(hanashite.SR48000), 1)
	panicOnError(err)

	if err != nil {
		panic(err)
	}

	sendingPipeline := hanashite.NewPipeline().
		AddProcessor(uvm.GetFrameProcessor()).
		AddProcessor(encoder.EncodeFrameProcessor()).
		AddProcessor(hanashite.EndTime).
		AddProcessor(udpproc.OutgoingFrameProcessor())


	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, os.Kill)




	rec := hanashite.NewRecorder(sendingPipeline)
	rec.StartRecording()

	decoder, err := pipeline.NewOpusDecoder(int(hanashite.SR48000), 1)
	panicOnError(err)

	player := pipeline.NewPortAudioPlayer()

	recv := hanashite.NewPipeline().
		AddProcessor(udpproc.IncomingFrameProcessor()).
		AddProcessor(decoder.DecodeFrameProcessor()).
		AddProcessor(player.PortAudioFrameProcessor())


	go func(){
		for true {
			recv.Process(&hanashite.AudioFrame{})
		}
	}()

	//FIXME make outgoing also a pipeline step

	for {
		select {
		case <-sig:
			fmt.Println("\nStopped Recording")
			rec.StopRecording() //FIXME wait for shutdown!
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

	hanashite.InitAudio()
	defer hanashite.TerminateAudio()

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

func recordAction(args []string, options map[string]string) int {
	/*outfile := args[0]
	hanashite.InitAudio()
	defer hanashite.TerminateAudio()

	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt, os.Kill)

	_, err := os.Stat(outfile)
	if !os.IsNotExist(err) {
		fmt.Printf("%s exists. Please choose different file\n", outfile)
		return -1
	}

	rec := hanashite.NewRecorder(outfile)

	fmt.Printf("\nRecording into \"%s\" press Ctrl+x to stop\n", outfile)
	rec.StartRecording()

	for {
		select {
		case <-sig:
			fmt.Println("\nStopped Recording")
			rec.StopRecording()
			return 0
		default:
		}
	}*/

	fmt.Println("Need to implement pipeleine with only wav out not enc out")
	return 0
}

func playerAction(args []string, options map[string]string) int {
	infile := args[0]
	hanashite.InitAudio()
	defer hanashite.TerminateAudio()

	_, err := os.Stat(infile)
	if os.IsNotExist(err) {
		fmt.Printf("%s does not exist. Exiting\n", infile)
		return -1
	}

	player := hanashite.NewPlayer()
	player.Play(infile)
	return 0
}

func encodeAction(args []string, options map[string]string) int {
	infile := args[0]
	outfile := args[1]

	enc := hanashite.NewOpusEncoder()
	enc.Encode(infile, outfile)

	return 0
}

func decodeAction(args []string, options map[string]string) int {
	infile := args[0]
	outfile := args[1]

	enc := hanashite.NewOpusEncoder()
	enc.Decode(infile, outfile)

	return 0
}

func Test() {
	hm := serialize.HanMessage{}

	hm.GetAuthResult()
}
