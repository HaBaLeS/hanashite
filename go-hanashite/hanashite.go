package hanashite

import (
	"encoding/json"
	"fmt"
	"github.com/habales/hanashite/go/serialize"
	"math/rand"
	"os"
	"os/user"
	"path"
)

type NetworkErrorCallback func(err error)
type NetworkCommendCallback func(msg *serialize.HanMessage)

type Config struct{
	Nickname string
	Server string
	Port string
	Channel string
}


func LoadConfig() (*Config, error){
	cf, err := getConfigPath()
	if err != nil {
		return nil,err
	}
	f, err := os.Open(cf)
	defer f.Close()
	if err != nil {
		return nil, err
	}
	dec := json.NewDecoder(f)
	ret := &Config{}
	err = dec.Decode(ret)
	return ret, err
}

func getConfigPath() (string, error) {
	u, err := user.Current()
	if err != nil {
		return "", err
	}
	cp := path.Join(u.HomeDir, ".hanashite")
	if _, err := os.Stat(cp); os.IsNotExist(err) {
		os.Mkdir(cp, os.ModePerm)
	}
	cf := path.Join(cp,"config.json")
	if _, err := os.Stat(cf); os.IsNotExist(err) {
		err := createDefaultFile(cf)
		if err != nil {
			return "", err
		}
	}
	return cf, nil
}

func createDefaultFile(cf string) error{
	def := Config{
		Server: "localhost",
		Port: "9876",
		Nickname: fmt.Sprintf("Anon-%d",rand.Intn(99999)),
	}
	f, err := os.Create(cf)
	defer f.Close()
	if err != nil {
		return err
	}
	enc := json.NewEncoder(f)
	enc.SetIndent("","   ")
	err = enc.Encode(def)
	if err != nil {
		return err
	}
	return nil
}

func UpdateConfig(cfg *Config) {
	fp, err := getConfigPath()
	if err != nil{
		panic(err)
	}
	f, err := os.Create(fp)
	defer f.Close()
	if err != nil {
		panic(err)
	}

	enc := json.NewEncoder(f)
	err = enc.Encode(cfg)
	if err != nil{
		panic(err)
	}
}