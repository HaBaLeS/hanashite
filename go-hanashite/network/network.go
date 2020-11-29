package network

import (
	"fmt"
	"github.com/golang/protobuf/proto"
	"github.com/google/uuid"
	hanashite "github.com/habales/hanashite/go"
	"github.com/habales/hanashite/go/serialize"
	"net"
	"time"
)

type Session struct {
	conn net.Conn
	errCb hanashite.NetworkErrorCallback
	cmdCb hanashite.NetworkCommendCallback
}



func NewSession(cfg *hanashite.Config,ecb hanashite.NetworkErrorCallback, ccb hanashite.NetworkCommendCallback) (*Session, error){
	session :=  &Session{}
	session.errCb = ecb
	session.cmdCb = ccb
	addr := fmt.Sprintf("%s:%s", cfg.Server, cfg.Port)
	conn, err  :=net.DialTimeout("tcp",addr , time.Second *5)
	if err != nil {
		return nil, err
	}
	session.conn= conn
	go session.handleIncomming()
	return session, nil
}

func (s *Session) Connect(user string) {
	ser := serialize.HanMessage{
		Msg: &serialize.HanMessage_Auth{
			Auth: &serialize.Auth{
				Username: user,
			},
		},
		MessageId: getMessageID(),
	}
	data, err := proto.Marshal(&ser)
	if err != nil {
		s.errCb(err)
		return
	}
	err = s.send(data)
	if err != nil {
		s.errCb(err)
		return
	}
}

func (s *Session) ListChannels(){
	hm := &serialize.HanMessage{
		Msg: &serialize.HanMessage_ChanLst{
			ChanLst: &serialize.ChannelList{},
		},
		MessageId:  getMessageID(),
	}

	data, err := proto.Marshal(hm)
	if err != nil {
		s.errCb(err)
	}
	err = s.send(data)
	if err != nil {
		if err != nil {
			s.errCb(err)
		}
	}
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

func (s *Session) Close() {
	s.conn.Close()
}

func (s *Session) handleIncomming() {
	buf := make([]byte, 4096)
	for {
		n, err := s.conn.Read(buf)
		if err != nil {
			s.errCb(err)
			continue
		}
		read := buf[:n]

		sh := serialize.StreamHeader{}
		err = proto.Unmarshal(read,&sh)
		if err != nil {
			s.errCb(err)
			continue
		}
		hm := &serialize.HanMessage{}
		err = proto.Unmarshal(read[10:10+sh.GetLength()], hm)
		if err != nil {
			s.errCb(err)
			continue
		}
		s.cmdCb(hm)
	}
}

func (s *Session) CreateChannel(arg string) {
	panic("CreateChannel not implemented")
}

func (s *Session) JoinChannel(channelID string){
	panic("CreateChannel not implemented")
}

func (s *Session) DeleteChannel(arg string) {
	panic("DeleteChannel not implemented")
}

func (s *Session) PartChannel(arg string) {
	panic("PartChannel not implemented")
}

func (s *Session) ChannelStatus(arg string) {
	panic("ChannelStatus not implemented")
}

func (s *Session) Status() {
	panic("Status not implemented")
}


func getMessageID() []byte {
	u, err  := uuid.NewRandom()
	if err != nil {
		panic(err)
	}
	b, err := u.MarshalBinary()
	if err != nil {
		panic(err)
	}
	return b
}