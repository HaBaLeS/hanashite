package hanashite

import (
	"encoding/binary"
	"fmt"
	"os"

	"github.com/hraban/opus"
)

const sampleRate = 48000
const channels = 1 // mono; 2 for stereo

type AudioEncoder struct {
}

func NewOpusEncoder() *AudioEncoder {
	return &AudioEncoder{}
}

func (aec *AudioEncoder) Test() {
	enc, err := opus.NewEncoder(sampleRate, channels, opus.AppVoIP)
	if err != nil {
		panic(err)
	}

	f, err := os.Open("test.wav")
	of, _ := os.Create("out.opus")
	defer of.Close()
	defer f.Close()
	pcm := make([]int16, 960)
	for true {
		if err := binary.Read(f, binary.LittleEndian, &pcm); err != nil {
			fmt.Println("binary.Read failed:", err)
			break
		}

		//var pcm []int16 =  // obtain your raw PCM data somewhere
		const bufferSize = 1000 // choose any buffer size you like. 1k is plenty.

		// Check the frame size. You don't need to do this if you trust your input.
		frameSize := len(pcm) // must be interleaved if stereo
		frameSizeMs := float32(frameSize) / channels * 1000 / sampleRate
		switch frameSizeMs {
		case 2.5, 5, 10, 20, 40, 60:
			// Good.
		default:
			fmt.Printf("Illegal frame size: %d bytes (%f ms)", frameSize, frameSizeMs)
			return
		}

		data := make([]byte, bufferSize)
		n, err := enc.Encode(pcm, data)
		if err != nil {
			panic(err)
		}
		data = data[:n] // only the first N bytes are opus data. Just like io.Reader.
		of.Write(data)
	}
}
