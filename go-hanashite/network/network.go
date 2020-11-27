package network

import (
	"github.com/golang/protobuf/proto"
	"github.com/habales/hanashite/go/serialize"
	"net"
)

type Session struct {
	//User

	//ActiveChannel

	conn net.Conn
}



func NewSession(server string) *Session{
	return &Session{}
}

func (s *Session) Connect(user string) error{
	ser := serialize.HanMessage{
		Msg: &serialize.HanMessage_Auth{
			Auth: &serialize.Auth{
				Username: user,
			},
		},
	}
	data, err := proto.Marshal(&ser)
	if err != nil {
		return err
	}
	return s.send(data)
}

func (s *Session) ListChannels(){

}

func (s *Session) JoinChannel(channelID string){

}

func (s *Session) send(data []byte) error {

	sh, err := proto.Marshal(&serialize.StreamHeader{
		Magic:  0x0008a71,
		Length: uint32(len(data)),
	})
	if err != nil {
		return err
	}
	_, err = s.conn.Write(append(sh, data...))
	if err != nil {
		return err
	}
	return nil
}
