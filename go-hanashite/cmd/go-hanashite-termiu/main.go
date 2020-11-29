package main

import (
	"fmt"
	ui "github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
	hanashite "github.com/habales/hanashite/go"
	"github.com/habales/hanashite/go/network"
	"github.com/habales/hanashite/go/serialize"
	"log"
	"strings"
)

type UI struct {
	session *network.Session
	cfg *hanashite.Config

	nickWg *widgets.Paragraph
	chanWg *widgets.Paragraph
	srvWg *widgets.Paragraph
	histWg *widgets.Paragraph
	inputWg *widgets.Paragraph
	uvmWg *widgets.BarChart
}



func main() {
	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()


	u := &UI{}
	cfg, le := hanashite.LoadConfig()
	if le != nil {
		panic(le)
	}
	u.cfg = cfg

	u.initWidgets()
	u.render()

	//UI Loop for  The Input widget
	uiEvents := ui.PollEvents()
	wgt := u.inputWg.Text
	for {
		e := <-uiEvents
		switch e.ID {
		case "<C-c>":
			if u.session != nil {
				u.session.Close()
			}
			return
		}
		if e.Type == ui.KeyboardEvent{
			r := []rune(wgt)
			if "<Space>" == e.ID {
				wgt = wgt + " "
			} else if "<Backspace>" == e.ID {
				if len(r) > 1 {
					wgt =string(r[:len(r)-1])
				}
			} else if "<Enter>" == e.ID {
				if len(r) > 1 {
					wgt = "#"
					u.processCommand(string(r[1:]))
				}
			} else {
				wgt = wgt + e.ID
			}
		}
		u.inputWg.Text=wgt
		u.render()
	}
}

type Command string

const (
	Help	Command = "help"
	List 	Command = "list"
	Create 	Command = "create"
	Delete	Command = "delete"
	Join	Command = "join"
	Part	Command = "part"
	Connect Command = "connect"
	Nick	Command = "nick"
	Server	Command = "server"
	Port	Command = "port"
	Save	Command = "save"
	MyStatus Command = "status"
	ChanStatus Command="channelstatus"

)

var help = `
HELP ... Abailable Commands:
    - /help 				#this
    - /list 				#List channels
    - /create <arg>			#Create channel
    - /join <arg>			#Join Channel
    - /part	<arg>			#Leave Channel
    - /connect 				#connect current server
    - /nick <arg>			#change Nickname
    - /server <arg>			#set servername
    - /port <arg>			#set port
    - /save 				#save nick, server, port
    - /status				#show status
    - /channelstatus <arg>	#channel status
`

func (u *UI)processCommand(input string) {
	defer func() {
		if r := recover(); r!=nil {
			u.writeToHistory(fmt.Sprintf("Error: %v", r),"red")
		}
	}()
	if strings.HasPrefix(input, "/"){
		//we try to parse a command
		 split := strings.Split(input[1:]," ")
		 cmd := split[0]
		 arg := ""
		 if len(split) >1 {
		 	arg = split[1]
		 }
		switch  Command(cmd) {
		case Help:
			 u.printHelp()
		case Connect:
			 u.connectToServer()
		case Nick:
			 u.updateNick(arg)
		case List:
			 u.session.ListChannels()
		case Create:
			 u.session.CreateChannel(arg)
		case Delete:
			 u.session.DeleteChannel(arg)
		case Join:
			 u.session.JoinChannel(arg)
		case Part:
			 u.session.PartChannel(arg)
		case ChanStatus:
			 u.session.ChannelStatus(arg)
		case MyStatus:
			 u.session.Status()
		case Server:
			 u.updateServer(arg)
		case Port:
			 u.updatePort(arg)
		case Save:
			 hanashite.UpdateConfig(u.cfg)
		default:
			 u.writeToHistory("#\"" + input + "\" unknown command", "red")
		}
	} else {
		u.writeToHistory(input, "magenta")
	}




}

func  (u *UI)connectToServer() {
	session, err := network.NewSession(u.cfg, func(err error) {
		u.writeToHistory(">>> E " + err.Error(),"red")
	}, func(msg *serialize.HanMessage) {
		u.handleIncomingCommand(msg)
	})
	if err != nil {
		u.writeToHistory(fmt.Sprintf("<<<%s", err), "yellow")
		return
	}
	u.session = session
	u.session.Connect(u.cfg.Nickname)
	u.writeToHistory("<<< Connected to " + u.cfg.Server, "green")
}

func (u *UI)writeToHistory(log, color string) {
	u.histWg.Text = fmt.Sprintf("[%s](fg:%s)\n%s",log,color, u.histWg.Text)
	u.render()
}

func (u *UI) render() {
	ui.Render(u.nickWg, u.chanWg, u.srvWg, u.histWg, u.uvmWg, u.inputWg)
}

func (u *UI) initWidgets() {
	u.nickWg = widgets.NewParagraph()
	u.nickWg.Title = "Nick"
	u.nickWg.Text = fmt.Sprintf("[%s](fg:green)",u.cfg.Nickname)
	u.nickWg.SetRect(0,0,20,3)

	u.chanWg = widgets.NewParagraph()
	u.chanWg.Title = "Channel"
	u.chanWg.Text = fmt.Sprintf("[%s](fg:green)",u.cfg.Channel)
	u.chanWg.SetRect(21,0,40,3)

	u.srvWg = widgets.NewParagraph()
	u.srvWg.Title = "Server"
	u.srvWg.Text = fmt.Sprintf("[%s](fg:green):[%s](fg:green)",u.cfg.Server, u.cfg.Port)
	u.srvWg.SetRect(41,0,80,3)

	u.histWg = widgets.NewParagraph()
	u.histWg.Title = "History"
	u.histWg.Text = help
	u.histWg.SetRect(7,3,80,21)
	u.histWg.BorderStyle.Fg = ui.ColorMagenta

	u.inputWg = widgets.NewParagraph()
	u.inputWg.Title = "Commands"
	u.inputWg.Text = "#"
	u.inputWg.SetRect(0, 21, 80, 24)
	u.inputWg.BorderStyle.Fg = ui.ColorYellow

	u.uvmWg = widgets.NewBarChart()
	u.uvmWg.Data = []float64{33}
	u.uvmWg.Labels = []string{"dbFS"}
	u.uvmWg.Title = "UV"
	u.uvmWg.SetRect(0, 3, 6, 21)
	u.uvmWg.BarWidth = 4
	u.uvmWg.BarColors = []ui.Color{ui.ColorRed, ui.ColorGreen}
	u.uvmWg.LabelStyles = []ui.Style{ui.NewStyle(ui.ColorBlue)}
	u.uvmWg.NumStyles = []ui.Style{ui.NewStyle(ui.ColorYellow)}
	u.uvmWg.MaxVal = 100
}

func (u *UI) updateNick(nick string) {
	enforceArg(nick)
	u.cfg.Nickname = nick
	u.nickWg.Text =   fmt.Sprintf("[%s](fg:green, mod:bold)",u.cfg.Nickname)
	u.writeToHistory("Set nick to: "+u.cfg.Nickname, "yellow")
}

func (u *UI) updateServer(arg string) {
	enforceArg(arg)
	u.cfg.Server= arg
	u.srvWg.Text = fmt.Sprintf("[%s](fg:green):[%s](fg:green)",u.cfg.Server, u.cfg.Port)
	u.writeToHistory("Server address updated", "yellow")
}

func (u *UI) updatePort(arg string) {
	enforceArg(arg)
	u.cfg.Port= arg
	u.srvWg.Text = fmt.Sprintf("[%s](fg:green):[%s](fg:green)",u.cfg.Server, u.cfg.Port)
	u.writeToHistory("Server address updated", "yellow")
}

func (u *UI) handleIncomingCommand(msg *serialize.HanMessage) {
	switch t := msg.Msg.(type) {
	case *serialize.HanMessage_AuthResult:
		u.handleAuthResult(t)
	case *serialize.HanMessage_ChanLstResult:
		u.handleChannelListResult(t)
	default:
		panic(fmt.Errorf("Not implemented Type: %v", t))
	}
}

func (u *UI) handleChannelListResult(t *serialize.HanMessage_ChanLstResult) {
	out := ""
	for _,v := range t.ChanLstResult.Channel {
		out += "- " +v.Name+"\n"
	}
	u.writeToHistory(">>> Available Channels: \n" + out, "green")
}

func (u *UI) handleAuthResult(t *serialize.HanMessage_AuthResult) {
	u.writeToHistory(">>> Connected", "green")
}

func (u *UI) printHelp() {
	u.writeToHistory(help, "yellow")
}


func enforceArg(arg string) {
	if arg == ""{
		panic("Argument needed")
	}
}
