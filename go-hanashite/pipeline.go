package hanashite

import (
	"fmt"
)

type Pipeline struct {
	steps []FrameProcessor
}



type FrameProcessor func(frame *AudioFrame)

func (p* Pipeline)Use(processor FrameProcessor){
	//Add processor to channel pipeline
	p.steps[0] = processor
}

func main() {
	pp := &Pipeline{}

	pp.Use(func(frame *AudioFrame){
		fmt.Println("lol")
	})

	pp.Use(EncodeOpus)
	pp.Use(WriteToWav)
	pp.Use(DetectActivity)
}

func EncodeOpus(frame *AudioFrame){

}

func WriteToWav(frame *AudioFrame){

}

func DetectActivity(frame *AudioFrame){

}