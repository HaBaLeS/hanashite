package main

import (
	"fmt"
	"net"

	"github.com/google/uuid"
	"github.com/habales/hanashite/go/serialize"
	"google.golang.org/protobuf/proto"
)

func main() {
	fmt.Println("Hello Rust Server")
	hm := serialize.HanMessage{
		Uuid: []byte(uuid.New().String()),
		Msg: &serialize.HanMessage_Auth{
			Auth: &serialize.Auth{Username: "ttp-1"},
		},
	}

	out, err := proto.Marshal(&hm)
	if err != nil {
		panic(err)
	}

	conn, err := net.Dial("tcp", "0.0.0.0:9876")
	if err != nil {
		panic(err)
	}
	fmt.Printf("%x", out)
	_, err = conn.Write(out)
	if err != nil {
		panic(err)
	}
	conn.Close()
}
