package hanashite

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"time"

	"github.com/gordonklaus/portaudio"
)



const (
	SR48000 float64 = 48000
	SR44100 float64 = 44100

	//MS2_5 int = 25
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
	pipeline *Pipeline

	frameNum uint64
}

type AudioFrame struct {
	Data16 []int16
	//Data32 []float32
	Encoded []byte
	EncBytes int

	FrameNum uint64

	Max int16 //calculate dB
	Min int16 //calculate dB
	Median int //calculate dB

	StartTime time.Time
	EndTime time.Time
	ProcTime time.Duration
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

func NewRecorder(pipeline *Pipeline) *Recorder {
	r :=  &Recorder{
		recording:   false,
		sampleRate:  SR48000,
		frameLength: MS20,
		channels:    1,
		pipeline:    pipeline,
		frameNum:    0,
	}
	r.frameBufferSize = sampleRate / 1000 * r.frameLength * channels
	return r
}


func (r *Recorder) StopRecording() {
	r.recording = false
}

func (r *Recorder) StartRecording() {
	f, err := os.Create(r.file) //FIXME this file needs to be put into the pipelie!!

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

			af := &AudioFrame{ //do we to create a frame pool?
				StartTime: time.Now(),
				Data16: make([]int16, r.frameBufferSize),
				FrameNum: r.frameNum,
			}
			err = stream.Read()
			if err != nil {
				panic(err)
			}
			r.frameNum++
			if err != nil {
				panic(err)
			}
			copy(af.Data16,in)

			r.pipeline.Process(af)
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
