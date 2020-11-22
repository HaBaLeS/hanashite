package pipeline

import (
	"fmt"
	"math"
)

const int16max float64 = 32768

//UVMeter is a pipeline step that calculates the dBFS (dB-FullScale) for the recorded audio signal
type UVMeter struct {

}

//NewUVMeter create a new Pipeline step UVMeter
func NewUVMeter() *UVMeter {
	return &UVMeter{}
}

//FrameProcessor return the FrameProcessor needed to UVMeter
func (u *UVMeter) FrameProcessor() FrameProcessor {
	return func(frame *AudioFrame) {
		u.detectActivity(frame)
	}
}

func  (u *UVMeter) detectActivity(frame *AudioFrame){
	min := frame.Data16[0]
	max := frame.Data16[0]
	var avg float64 = 0

	for _,v := range frame.Data16 {
		if v <= min {
			min = v
		} else {
			max = v
		}
		avg += float64(v)
	}
	frame.Min = min
	frame.Max = max

	amed := math.Abs(avg / float64(len(frame.Data16)))

	//amplitude = 20 * log10(abs(sample) / 32767) dBFS
	dbfs := int(20 * math.Log10(amed/int16max))

	fmt.Printf("%d;\tStat: %d\n", frame.FrameNum, dbfs)
}