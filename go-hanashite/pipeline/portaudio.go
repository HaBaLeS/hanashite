package pipeline

import (
	"fmt"
	"github.com/gordonklaus/portaudio"
)

//PortAudioProcessor is a Pipeline step to record/play PCM as end/start of the Pipeline
type PortAudioProcessor struct {
	outBuf []int16
	outStream *portaudio.Stream

	inBuf []int16
	inStream *portaudio.Stream
	inFrameBufferSize int

	sampleRate float64
	running bool

}

//NewPortAudioProcessor create a new Player pipeline step for record and play PCM
func NewPortAudioProcessor(frameLength float64) *PortAudioProcessor{
	sampleRate := 48000.0
	inFrameBufferSize := int(sampleRate / 1000 * frameLength * 1)
	switch frameLength {
		case 2.5,5, 10, 20, 40, 60:
			//all good
			break
		default:
			panic("frameLength: must be in: 2.5,5, 10, 20, 40, 60")
	}
	fmt.Printf("Recording samples of %fms with %fHz at 16Bit", frameLength,sampleRate)

	pap := &PortAudioProcessor{
		outBuf:  make([]int16, inFrameBufferSize),
		inBuf: make([]int16, inFrameBufferSize),
		inFrameBufferSize: inFrameBufferSize,
		sampleRate: sampleRate,
		running:  true,
	}

	return pap
}

//PlayerFrameProcessor wraps the FrameProcessor for playing PCM
func (p *PortAudioProcessor) PlayerFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		p.play(frame)
	}
}

//RecorderFrameProcessor wraps the FrameProcessor for playing PCM
func (p *PortAudioProcessor) RecorderFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		p.record(frame)
	}
}

func (p *PortAudioProcessor) play(frame *AudioFrame) {
	if !p.running {
		return
	}
	p.outBuf = frame.Data16
	err := p.outStream.Write()
	//panicIfError(err)
	if err != nil {
		fmt.Println(err)
	}
}

func (p *PortAudioProcessor) record(frame *AudioFrame) {
	if !p.running {
		return
	}

	frame.Data16 = make([]int16, p.inFrameBufferSize)
	p.inStream.Read()
	copy(frame.Data16,p.inBuf)
}

//Shutdown closes all streams
func (p *PortAudioProcessor) Shutdown() {
	p.running = false
	p.inStream.Stop()
	p.inStream.Close()

	p.outStream.Stop()
	p.outStream.Close()
}

//InitAudio initializes PortAudio and must be calles before any other Audio Operation
func (p *PortAudioProcessor) InitAudio() {
	fmt.Println("Initializing Initializing")
	portaudio.Initialize()

	outStream, err := portaudio.OpenDefaultStream(0, 1, p.sampleRate, len(p.outBuf), &p.outBuf)
	panicIfError(err)
	p.outStream = outStream
	panicIfError(p.outStream.Start())

	inStream, err := portaudio.OpenDefaultStream(1, 0, p.sampleRate, len(p.inBuf), &p.inBuf)
	panicIfError(err)
	p.inStream = inStream
	panicIfError(p.inStream.Start())

}

//TerminateAudio Terminates PortAudio and must be called before hanashite is shut down
func (p *PortAudioProcessor) TerminateAudio() {
	fmt.Println("Terminating PortAudio")
	portaudio.Terminate()
}