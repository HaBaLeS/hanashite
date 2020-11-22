package pipeline

import "time"

//AudioFrame is the struct traveling through the pipeline
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


//Pipeline is a chain of FrameProcessor functions that get call in the order they are added to
type Pipeline struct {
	first *pipelineStep
	last *pipelineStep
}

//NewPipeline creates a new Pipeline where the FrameProcessor can be added. There is one Pipeline for Rec-> Send and one for Rec -> Play
func NewPipeline() *Pipeline {
	pp := &Pipeline{}
	return pp
}

//Process adds the forst AudioFrame to the Pipeline
func (p *Pipeline) Process(frame *AudioFrame){
	p.first.in <- frame
}

//FrameProcessor is a function that takes a AudioFrame and does something with it. Once it returns the next step in
//the Pipeline is executed. All Coded executed here adds to the Delay in the communication
type FrameProcessor func(frame *AudioFrame)

//AddProcessor adds a FrameProcessor to the end of the Pipeline
func (p*Pipeline)AddProcessor(processor FrameProcessor) *Pipeline {
	if p.first == nil {
		p.first = &pipelineStep{
			in :  make(chan *AudioFrame,1024), //Buffer 1024 frames
			pf : processor,
		}
		go p.first.start()
		p.last = p.first
	} else {
		next := &pipelineStep{
			in :  make(chan *AudioFrame),
			pf : processor,
		}
		p.last.next =next
		p.last = next
		go next.start()
	}
	return p
}

type pipelineStep struct{
	in   chan *AudioFrame
	pf   FrameProcessor
	next *pipelineStep
}

func (ps *pipelineStep) start(){
	for true {
		af := <- ps.in
		ps.pf(af)
		if ps.next != nil {
			ps.next.in <- af
		}
	}
}



