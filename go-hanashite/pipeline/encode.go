package pipeline

import (
	"encoding/binary"
	"fmt"
	"github.com/hraban/opus"
)

//OpusCodec is a Pipeline step to encode/decode Opus
type OpusCodec struct {
	encoder *opus.Encoder
	decoder *opus.Decoder
	sampleRate int
	channels int
}

//NewOpusCodec create a new Pipeline step to encode/decode PCM to OPUS frames
func NewOpusCodec(sampleRate, channels int) (*OpusCodec, error){
	enc, err := opus.NewEncoder(sampleRate, channels, opus.AppVoIP)
	panicIfError(err)
	dec, err := opus.NewDecoder(sampleRate, channels)
	panicIfError(err)
	if err != nil {
		return nil, err
	}
	return &OpusCodec{
		encoder: enc,
		decoder: dec,
		channels: channels,
		sampleRate: sampleRate,
	}, nil
}

//EncodeFrameProcessor returns the FrameProcessor to Encode PCM frames
func (oe *OpusCodec) EncodeFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		oe.encode(frame)
	}
}

//DecodeFrameProcessor returns the FrameProcessor to decode Opus to PCM
func (oe *OpusCodec) DecodeFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		oe.decode(frame)
	}
}

func (oe *OpusCodec) decode(frame *AudioFrame) {

	//FIXME drop frame if frameNum is before the last one!!!
	//Store info to drop in frame

	frame.Data16 =  make([]int16, 4096)
	n, dee := oe.decoder.Decode(frame.Encoded, frame.Data16)
	if dee != nil {
		panic(dee)
	}
	frame.Data16 = frame.Data16[:n]
	binary.Size(frame.Data16)
	fmt.Printf("DecodePCM with size: %d\n", binary.Size(frame.Data16))
}

func (oe *OpusCodec) encode(frame *AudioFrame) {

	pcm := frame.Data16
	//var pcm []int16 =  // obtain your raw PCM data somewhere
	const bufferSize = 2048 // choose any buffer size you like. 1k is plenty.

	//FIXME ... this is debug code

	// Check the frame size. You don't need to do this if you trust your input.
	frameSize := len(pcm) // must be interleaved if stereo
	frameSizeMs := frameSize / oe.channels * 1000 / oe.sampleRate
	switch frameSizeMs {
		case /*2.5,*/ 5, 10, 20, 40, 60:
			// Good.
		default:
			fmt.Printf("Illegal frame size: %d bytes (%d ms)", frameSize, frameSizeMs)
			return
	}
	//End Debug

	frame.Encoded = make([]byte, bufferSize)
	n, err := oe.encoder.Encode(pcm, frame.Encoded)
	frame.EncBytes = n
	panicIfError(err)
	fmt.Printf("encoded sample size: %d\n", n )
}