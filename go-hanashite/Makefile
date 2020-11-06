VERSION := 0.0.1 #add git tag after first tag is created
BUILD := $(shell git rev-parse --short HEAD)
LDFLAGS= -X=main.Version=$(VERSION) -X=main.Build=$(BUILD)
CHANGES := $(shell test -n "$$(git status --porcelain)" && echo '+CHANGES' || true)

TARGET := hanashite-cli

OS := linux windows darwin
ARCH := amd64

build: vet lint
	go build  -ldflags "$(LDFLAGS)" -o $(TARGET) cmd/go-hanashite-cli/main.go 
	
	#Cross compile wth CGO enabled requires to set up compilers and SDK's ... will do that later in a VM or container
	#https://subscription.packtpub.com/book/application_development/9781789138412/app02/app02lvl1sec96/cross-compiling-for-macos-with-cgo
	#CGO_ENABLED=1 GOOS=linux GOARCH=amd64 go build  -ldflags "$(LDFLAGS)" -o $(TARGET) cmd/go-hanashite-cli/main.go 
	#CGO_ENABLED=1 GOOS=windows GOARCH=amd64 go build  -ldflags "$(LDFLAGS)" -o $(TARGET).exe cmd/go-hanashite-cli/main.go 
	#CGO_ENABLED=1 GOOS=darwin GOARCH=amd64 go build  -ldflags "$(LDFLAGS)" -o $(TARGET).dmg cmd/go-hanashite-cli/main.go 

clean:
	go clean -i ./...
	rm hanashite-cli

vet:
	go vet ./...

lint:
	golint ./...

tools:
	#go get -u github.com/mitchellh/gox
	go get -u golang.org/x/lint/golint