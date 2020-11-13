package main

import (
	"fmt"
	"net"
	"os"

	"github.com/golang/protobuf/proto"
	"github.com/habales/hanashite/go/serialize"
	"github.com/teris-io/cli"
)

func main() {

	app := cli.New("Hanashite Server go-version").WithArg(cli.NewArg("addr", "Host and port.")).WithAction(startServer)

	os.Exit(app.Run(os.Args, os.Stdout))

}

func startServer(args []string, options map[string]string) int {
	l, err := net.Listen("tcp", args[0])
	panicIfError(err)

	for {
		conn, err := l.Accept()
		panicIfError(err)
		go handleIncoming(conn)
	}

}

func panicIfError(err error) {
	if err != nil {
		panic(err)
	}
}

func handleIncoming(conn net.Conn) {
	fmt.Printf("Incoming client connection: %s\n", conn.LocalAddr())
	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	panicIfError(err)
	msg := &serialize.HanMessage{}
	err = proto.Unmarshal(buf[:n], msg)
	panicIfError(err)
	fmt.Printf("Mfg: %s", string(msg.GetAuth().Username))
}
