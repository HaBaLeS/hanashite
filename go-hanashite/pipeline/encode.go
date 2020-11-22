package pipeline

import (
	"encoding/binary"
	"fmt"
	hanashite "github.com/habales/hanashite/go"
	"github.com/hraban/opus"
)

type OpusEncoder struct {
	encoder *opus.Encoder
	sampleRate int
	channels int
}

type OpusDecoder struct {
	decoder *opus.Decoder
	sampleRate int
	channels int
}

func NewOpusEncoder(sampleRate, channels int) (*OpusEncoder, error){
	enc, err := opus.NewEncoder(sampleRate, channels, opus.AppVoIP)
	if err != nil {
		return nil, err
	}
	return &OpusEncoder{
		encoder: enc,
		channels: channels,
		sampleRate: sampleRate,
	}, nil
}

func NewOpusDecoder(sampleRate, channels int) (*OpusDecoder, error){
	enc, err := opus.NewDecoder(sampleRate, channels)
	if err != nil {
		return nil, err
	}
	return &OpusDecoder{
		decoder: enc,
		channels: channels,
		sampleRate: sampleRate,
	}, nil
}

func (oe *OpusEncoder) EncodeFrameProcessor() hanashite.FrameProcessor {
	return func(frame *hanashite.AudioFrame){
		oe.encode(frame)
	}
}

func (oe *OpusDecoder) DecodeFrameProcessor() hanashite.FrameProcessor {
	return func(frame *hanashite.AudioFrame){
		oe.decode(frame)
	}
}

func (oe *OpusDecoder) decode(frame *hanashite.AudioFrame) {

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

func (oe *OpusEncoder) encode(frame *hanashite.AudioFrame) {

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