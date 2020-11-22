package hanashite

import (
	"time"
)

type Pipeline struct {
	first *PipelineStep
	last *PipelineStep
}


type PipelineStep struct{
	in   chan *AudioFrame
	pf   FrameProcessor
	next *PipelineStep
}

func (ps *PipelineStep) Start(){
	for true {
		af := <- ps.in
		ps.pf(af)
		if ps.next != nil {
			ps.next.in <- af
		}
	}
}


type FrameProcessor func(frame *AudioFrame)

func (p*Pipeline)AddProcessor(processor FrameProcessor) *Pipeline {
	if p.first == nil {
		p.first = &PipelineStep{
			in :  make(chan *AudioFrame,1024), //Buffer 1024 frames
			pf : processor,
		}
		go p.first.Start()
		p.last = p.first
	} else {
		next := &PipelineStep{
			in :  make(chan *AudioFrame),
			pf : processor,
		}
		p.last.next =next
		p.last = next
		go next.Start()
	}
	return p
}

func (p *Pipeline) Process(frame *AudioFrame){
	//fmt.Printf("%d:\tStart\n",frame.FrameNum)
	p.first.in <- frame
}

func NewPipeline()  *Pipeline {
	pp := &Pipeline{}
	return pp
}

func EndTime(frame *AudioFrame) {
	frame.EndTime = time.Now()
	frame.ProcTime = frame.EndTime.Sub(frame.StartTime)
}
