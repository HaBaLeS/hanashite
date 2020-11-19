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
     ./hanashite-cli devicelist


### Usage:

    Description:
        Commandline Version of go-hanashite

    Sub-commands:
         hanashite-cli list      Enumerate Devices
         hanashite-cli version   Show Version information
         hanashite-cli record    Record raw pcm file
         hanashite-cli encode    Encode raw file to opus
         hanashite-cli decode    Decode opus frames to raw pcm
         hanashite-cli play      Play raw pcm file


