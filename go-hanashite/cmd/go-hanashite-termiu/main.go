// Copyright 2017 Zack Guo <zack.y.guo@gmail.com>. All rights reserved.
// Use of this source code is governed by a MIT license that can
// be found in the LICENSE file.


package main

import (
	"fmt"
	ui "github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
	"github.com/habales/hanashite/go/network"
	"log"
	"strings"
)

type UI struct {
	history *widgets.Paragraph
	session *network.Session
	user string
}



func main() {
	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()

//80x24

	u := &UI{
		user: "Anon",
	}

	p2 := widgets.NewParagraph()
	p2.Title = "Commands"
	p2.Text = "#"
	p2.SetRect(0, 21, 80, 24)
	p2.BorderStyle.Fg = ui.ColorYellow


	u.history = widgets.NewParagraph()
	u.history.Title = "History"
	u.history.Text = ""
	u.history.SetRect(7,0,80,21)
	u.history.BorderStyle.Fg = ui.ColorMagenta





	bc := widgets.NewBarChart()
	bc.Data = []float64{33}
	bc.Labels = []string{"dbFS"}
	bc.Title = "UV"
	bc.SetRect(0, 0, 6, 21)
	bc.BarWidth = 4
	bc.BarColors = []ui.Color{ui.ColorRed, ui.ColorGreen}
	bc.LabelStyles = []ui.Style{ui.NewStyle(ui.ColorBlue)}
	bc.NumStyles = []ui.Style{ui.NewStyle(ui.ColorYellow)}
	bc.MaxVal = 100



	ui.Render( bc, p2, u.history)



	uiEvents := ui.PollEvents()
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
			r := []rune(p2.Text)
			if "<Space>" == e.ID {
				p2.Text = p2.Text + " "
			} else if "<Backspace>" == e.ID {

				if len(r) > 1 {
					p2.Text =string(r[:len(r)-1])
				}

			} else if "<Enter>" == e.ID {
				if len(r) > 1 {
					p2.Text = "#"
					u.processCommand(string(r[1:]))
				}
			} else {
				p2.Text = p2.Text + e.ID
			}

		}

		ui.Render( bc, p2, u.history)

	}
}

func (u *UI)processCommand(cmd string) {
	u.writeToHistory(">>> " + cmd,"red")
	if strings.HasPrefix(cmd, "connect ") {
		go u.connectToServer(strings.TrimLeft(cmd, "connect"))
	} else if strings.HasPrefix(cmd, "nick ") {
		usr := strings.TrimLeft(cmd, "nick")
		usr = strings.TrimSpace(usr)
		u.user = usr
		u.writeToHistory("Set user to: "+u.user, "green")
	} else {
		go u.writeToHistory("<<<\"" + cmd + "\" unknown command", "yellow")
	}

}

func  (u *UI)connectToServer(arg string) {
	arg = strings.TrimSpace(arg)
	session, err := network.NewSession(arg)
	if err != nil {
		u.writeToHistory(fmt.Sprintf("<<<%s", err), "yellow")
		return
	}
	u.session = session
	err = u.session.Connect(u.user)
	u.writeToHistory("<<< Connected to " + arg, "green")
	if err != nil {
		u.writeToHistory(fmt.Sprintf("<<<%s", err), "yellow")
		return
	}
}

func (u *UI)writeToHistory(log, color string) {
	u.history.Text = fmt.Sprintf("[%s](fg:%s)\n%s",log,color, u.history.Text)
	ui.Render(u.history)
}
