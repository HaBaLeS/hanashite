# hanashite

## Golang

### Prequisite:

Install C Libraries
#### Ubuntu
    
     portaudio19-dev libopusfile-dev libopus-dev

#### Correct GO setup
Make sure you have your GOPATH:/bin exported in your PATH while calling Make. 


### On Linux do:

     make tools #needed only once
     make build
     ./hanashite-cli version
     
     #Warning, use a headset! No echo cancelation yet!
     ./hanashite-cli pipeline
     
     


### Usage:

hanashite-cli

	Description:
	    Commandline Version of go-hanashite

	Sub-commands:
	    hanashite-cli list       Enumerate Devices
	    hanashite-cli version    Show Version information
	    hanashite-cli pipeline   Start full pipeline


