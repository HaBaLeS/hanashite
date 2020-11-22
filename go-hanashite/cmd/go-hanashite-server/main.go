package main

import (
	"fmt"
	"github.com/golang/protobuf/proto"
	"github.com/google/uuid"
	"github.com/habales/hanashite/go/serialize"
	"github.com/teris-io/cli"
	"net"
	"os"
)

type ClientJob struct {
	name string
	conn net.Conn
}

func main() {

	app := cli.New("Hanashite Server go-version").WithArg(cli.NewArg("addr", "Host and port.")).WithAction(startServer)

	os.Exit(app.Run(os.Args, os.Stdout))

}

func startServer(args []string, options map[string]string) int {

	//clientJobs := make(chan ClientJob)
	//go handleIncoming(clientJobs)

	l, err := net.Listen("tcp", args[0])
	panicIfError(err)
	a, e := net.ResolveUDPAddr("udp","0.0.0.0:9876")
	if e != nil {
		panic(e)
	}
	uc, ex := net.ListenUDP("udp",  a)
	if ex != nil {
		panic(ex)
	}
	go handleUDP(uc)

	for {
		conn, err := l.Accept()
		panicIfError(err)
		go handleIncoming(conn)
	}

}

func handleUDP(uc *net.UDPConn) {
	for {
		buf := make([]byte, 4096)
		n, addr, err := uc.ReadFromUDP(buf)
		panicIfError(err)
		sh := &serialize.StreamHeader{}
		ap := &serialize.HanUdpMessage{}


		//get stream header
		err = proto.Unmarshal(buf[:10], sh)
		panicIfError(err)
		//fmt.Printf("Packet Read: %d, Stream header Size: %d from: %s\n", n, sh.GetLength(), addr.String())

		err = proto.Unmarshal(buf[10:10+sh.GetLength()], ap)
		panicIfError(err)

		fmt.Printf("Received: %d\n", n)

		x, err := uc.WriteToUDP(buf[:n], addr)
		panicIfError(err)
		fmt.Printf("Sending back: %d\n", x)
	}

}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}

func handleIncoming(conn net.Conn) {
	for {

		fmt.Printf("Incoming client connection: %s\n", conn.LocalAddr())
		buf := make([]byte, 1024)
		n, err := conn.Read(buf)
		panicIfError(err)
		sh := &serialize.StreamHeader{}
		msg := &serialize.HanMessage{}
		err = proto.Unmarshal(buf[:10], sh)

		fmt.Printf("Packet Read: %d, Stream header Size: %d\n", n, sh.GetLength())

		panicIfError(err)
		err = proto.Unmarshal(buf[10:10+sh.GetLength()], msg)
		panicIfError(err)
		uuidd, err := uuid.FromBytes(msg.GetUuid())
		panicIfError(err)
		fmt.Printf("%s: %s\n ", uuidd.String(), string(msg.GetAuth().Username))

	}
}
