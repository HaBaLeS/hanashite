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

func (aec *AudioEncoder) Encode(in, out string) {
	enc, err := opus.NewEncoder(sampleRate, channels, opus.AppVoIP)
	if err != nil {
		panic(err)
	}

	f, err := os.Open(in)
	of, _ := os.Create(out)
	defer of.Close()
	defer f.Close()
	pcm := make([]int16, 960)
	for true {
		if err := binary.Read(f, binary.BigEndian, &pcm); err != nil {
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
		fmt.Printf("Packet: %d bytes\n", n)
		err = binary.Write(of, binary.BigEndian, int32(n))
		if err != nil {
			panic(err)
		}
		err = binary.Write(of, binary.BigEndian, data)
		if err != nil {
			panic(err)
		}

	}

}

func (aec *AudioEncoder) Decode(in, out string) {
	is, err := os.Open(in)
	defer is.Close()
	if err != nil {
		panic(err)
	}

	os, err := os.Create(out)
	defer os.Close()

	dec, err := opus.NewDecoder(sampleRate, channels)
	if err != nil {
		panic(err)
	}

	data := make([]int16, 4096)
	var n int32
	for true {
		if err := binary.Read(is, binary.BigEndian, &n); err != nil {
			fmt.Println("binary.Read failed:", err)
			break
		}
		buf := make([]byte, n)
		rn, err := is.Read(buf)
		if err != nil {
			panic(err)
		}
		if int32(rn) != n {
			panic("Out of sync!")
		}
		p, dee := dec.Decode(buf, data)
		if dee != nil {
			panic(dee)
		}
		binary.Write(os, binary.BigEndian, data[:p])
	}
}
