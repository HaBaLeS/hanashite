package hanashite

import (
	"fmt"
	"time"
)

type Pipeline struct {
	first *PipelineStep
	last *PipelineStep
}


type PipelineStep struct{
	in  chan *AudioFrame
	pf FrameProcessor
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

func (p* Pipeline)Use(processor FrameProcessor){
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
}

func (p *Pipeline) Process(frame *AudioFrame){
	fmt.Printf("%d:\tStart\n",frame.FrameNum)
	p.first.in <- frame
}

func NewPipeline()  *Pipeline{
	pp := &Pipeline{}


	pp.Use(EncodeOpus)
	pp.Use(WriteToWav)
	pp.Use(DetectActivity)
	pp.Use(EndTime)
	pp.Use(SendUdp)

	return pp
}

func EndTime(frame *AudioFrame) {
	frame.EndTime = time.Now()
	frame.ProcTime = frame.EndTime.Sub(frame.StartTime)
}



func SendUdp(frame *AudioFrame) {
	fmt.Printf("%d:\tSend UDP. ProcTime: %d ms\n",frame.FrameNum, frame.ProcTime.Milliseconds())
}

func EncodeOpus(frame *AudioFrame){

}

func WriteToWav(frame *AudioFrame){

}

func DetectActivity(frame *AudioFrame){
	min := frame.Data16[0]
	max := frame.Data16[0]
	var avg int = 0

	for _,v := range frame.Data16 {
		if v <= min {
			min = v
		} else {
			max = v
		}
		avg += int(v)
	}
	frame.min = min
	frame.max = max
	frame.median = avg / len(frame.Data16)
	fmt.Printf("%d;\tStat: %d, %d, %d\n", frame.FrameNum, frame.min, frame.max, frame.max)
}