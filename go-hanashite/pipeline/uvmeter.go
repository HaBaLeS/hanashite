package pipeline

import (
	"fmt"
	"github.com/habales/hanashite/go"
	"math"
)

const int16max float64 = 32768

type UVMeter struct {

}

func NewUVMeter() *UVMeter {
	return &UVMeter{}
}

func (u *UVMeter) GetFrameProcessor() hanashite.FrameProcessor {
	return func(frame *hanashite.AudioFrame) {
		u.detectActivity(frame)
	}
}

func  (u *UVMeter) detectActivity(frame *hanashite.AudioFrame){
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