package config

import (
	"gitlab.com/bloom42/libs/sane-go"
	"io/ioutil"
)

var DisposableEmailDomains map[string]bool

var Config Configuration

type Configuration struct {
	Port     uint16         `sane:"port"`
	Env      string         `sane:"env"`
	Database DatabaseConfig `sane:"database"`
}

type DatabaseConfig struct {
	URL string `sane:"url"`
}

func Init(configFile string) error {
	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		return err
	}

	err = sane.Unmarshal(data, &Config)
	if err != nil {
		return err
	}

	// TODO
	DisposableEmailDomains = map[string]bool{}

	return err
}
