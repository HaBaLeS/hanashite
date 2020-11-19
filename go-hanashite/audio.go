package hanashite

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"

	"github.com/gordonklaus/portaudio"
)



const (
	SR48000 float64 = 48000
	SR44100 float64 = 44100

	MS2_5 int = 2.5
	MS5 int = 5
	MS10 int = 10
	MS20 int = 20
	MS40 int = 40
	MS60 int = 60
)

type Recorder struct {
	recording       bool
	file            string
	sampleRate      float64
	frameLength     int
	frameBufferSize int
	channels        int
}

type AudioFrame struct {
	Data16 []int16
	Data32 []float32

	max float32 //calculate dB
	min float32 //calculate dB
	median float32 //calculate dB
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
	r :=  &Recorder{
		recording:   false,
		file:        file,
		sampleRate:  SR48000,
		frameLength: MS20,
		channels:    1,

	}
	r.frameBufferSize = sampleRate / 1000 * r.frameLength * channels
	return r
}


func (r *Recorder) StopRecording() {
	r.recording = false
}

func (r *Recorder) StartRecording() {
	f, err := os.Create(r.file)

	//The latency in milliseconds due to this buffering  is:
	//latency_msec = 1000 * channels * framesPerBuffer / framesPerSecond

	in := make([]int16, r.frameBufferSize) //check Portaudio.go#sampleFormat for the available formats
	stream, err := portaudio.OpenDefaultStream(r.channels, 0, r.sampleRate, r.frameBufferSize, in)
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
