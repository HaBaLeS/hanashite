package pipeline

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"github.com/habales/hanashite/go/serialize"
	"google.golang.org/protobuf/proto"
	"net"
)

//MaxUDPPacketSize defined the save size of a UDP packet to be guranteed to not get fragmented. See:
//https://stackoverflow.com/questions/1098897/what-is-the-largest-safe-udp-packet-size-on-the-internet for Detaiils
const MaxUDPPacketSize = 508


//UDPProcessor is a Pipeline step to send and receive UDP Audio Frames
type UDPProcessor struct {
	conn net.Conn
	userID []byte
	channelID []byte

}

//NewUDPProc creates a new UDPProcessor responsible for Sending frames to a server and receiving them.
func NewUDPProc(addr string) (*UDPProcessor, error){
	conn, err :=  net.Dial("udp",addr)
	if err != nil {
		return nil, err
	}

	return &UDPProcessor{
		conn: conn,
		userID: []byte("falko"),
		channelID: []byte("secret world"),
	}, nil
}

//OutgoingFrameProcessor wraps the FrameProcessor function for sending
func (u *UDPProcessor) OutgoingFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		u.processOutgoing(frame)
	}
}

//IncomingFrameProcessor wraps the FrameProcessor function for receiving
func (u *UDPProcessor) IncomingFrameProcessor() FrameProcessor {
	return func(frame *AudioFrame){
		u.processIncomming(frame)
	}
}

func (u *UDPProcessor) processOutgoing(frame *AudioFrame) {
	data := serialize.HanUdpMessage{
		UserId: u.userID,
		AudioFrame: &serialize.AudioPacket{
			ChannelId: u.channelID,
			SequernceId: frame.FrameNum,
			Data: frame.Encoded[:frame.EncBytes],
		},
	}
	out1, err := proto.Marshal(&data)
	if err != nil {
		panicIfError(err)
	}
	sh := serialize.StreamHeader{
		Magic: 0x0008a71,
		Length:  uint32(len(out1)),
	}
	out2, err := proto.Marshal(&sh)
	n1, err := u.conn.Write(append(out2, out1...))
	panicIfError(err)
	if n1 > MaxUDPPacketSize {
		panic("Sent a packet that could be fragmented!!")
	}
	fmt.Printf("Send Frame of lenght: %d bytes\n", n1)
}

func (u *UDPProcessor) processIncomming(frame *AudioFrame) {

	//FIXME assuming we always get full packages!!!!
	buf := make([]byte, MaxUDPPacketSize)
	n, err := u.conn.Read(buf)
	panicIfError(err)

	sh := &serialize.StreamHeader{}
	ap := &serialize.HanUdpMessage{}


	//get stream header
	err = proto.Unmarshal(buf[:10], sh)
	panicIfError(err)

	err = proto.Unmarshal(buf[10:10+sh.GetLength()], ap)
	panicIfError(err)

	frame.Encoded = ap.AudioFrame.Data
	frame.FrameNum = ap.AudioFrame.SequernceId

	fmt.Printf("%d\tReceived: %d bytes in a packet\n",frame.FrameNum,n)
}

//FIXME useful conversion, but not used ... move to util or discard
func bytesFromInt16(data16 []int16) []byte {
	buf := new(bytes.Buffer)
	binary.Write(buf, binary.BigEndian, data16)
	return buf.Bytes()
}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}