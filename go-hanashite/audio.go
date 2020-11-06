package hanashite

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"

	"github.com/gordonklaus/portaudio"
)

type Recorder struct {
	recording bool
	file      string
}

type Player struct {
}

func InitAudio() {
	fmt.Println("Terminating Initializing")
	portaudio.Initialize()
}

func TerminateAudio() {
	fmt.Println("Terminating PortAudio")
	portaudio.Terminate()
}

func NewRecorder(file string) *Recorder {
	return &Recorder{
		recording: false,
		file:      file,
	}
}

func (r *Recorder) StopRecording() {
	r.recording = false
}

func (r *Recorder) StartRecording() {
	f, err := os.Create(r.file)

	//The latency in milliseconds due to this buffering  is:
	//latency_msec = 1000 * numBuffers * framesPerBuffer / framesPerSecond
	numInputChannels := 1
	framesPerBuffer := 64
	sampleRate := 48000.0
	in := make([]int16, framesPerBuffer)
	stream, err := portaudio.OpenDefaultStream(numInputChannels, 0, sampleRate, framesPerBuffer, in)
	if err != nil {
		panic(err)
	}

	err = stream.Start()
	if err != nil {
		panic(err)
	}

	r.recording = true
	go func() {
		for r.recording {
			err = stream.Read()
			err = binary.Write(f, binary.BigEndian, in)
			//nSamples += len(in)
			fmt.Print(".")
		}
		stream.Close()
		f.Close()
	}()

}

func NewPlayer() *Player {
	return &Player{}
}

func (p *Player) Play(file string) {

	f, _ := os.Open(file)

	out := make([]int16, 64)
	stream, err := portaudio.OpenDefaultStream(0, 1, 48000, len(out), &out)
	if err != nil {
		panic(err)
	}

	defer stream.Close()
	err = stream.Start()
	if err != nil {
		panic(err)
	}
	defer stream.Stop()

	for true {
		fmt.Print(".")
		err := binary.Read(f, binary.BigEndian, out)
		if err == io.EOF {
			break
		}
		err = stream.Write()
		if err != nil {
			panic(err)
		}

	}

}
