package pipeline

import (
	"fmt"
	"github.com/gordonklaus/portaudio"
	hanashite "github.com/habales/hanashite/go"
)

type Player struct {
	outBuf []int16
	outStream *portaudio.Stream
}

func NewPortAudioPlayer() *Player{

	pap := &Player{
		outBuf:  make([]int16, 20),

	}
	stream, err := portaudio.OpenDefaultStream(0, 1, 48000, len(pap.outBuf), &pap.outBuf)
	pap.outStream = stream
	panicIfError(err)
	pap.outStream.Start()
	return pap
}

func (p *Player) PortAudioFrameProcessor() hanashite.FrameProcessor {
	return func(frame *hanashite.AudioFrame){
		p.play(frame)
	}
}

func (p *Player) play(frame *hanashite.AudioFrame) {
	p.outBuf = frame.Data16
	err := p.outStream.Write()
	//panicIfError(err)
	if err != nil {
		fmt.Println(err)
	}

}