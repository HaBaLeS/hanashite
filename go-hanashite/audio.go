package hanashite

import "github.com/gordonklaus/portaudio"

func InitAudio() {
	portaudio.Initialize()
}

func TerminateAudio() {
	portaudio.Terminate()
}
