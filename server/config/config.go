package config

import (
	"gitlab.com/bloom42/libs/sane-go"
	"io/ioutil"
)

var DisposableEmailDomains map[string]bool
var Database DatabaseConfig
var Port uint16
var Env string

type configuration struct {
	Port     uint16         `sane:"port"`
	Env      string         `sane:"env"`
	Database DatabaseConfig `sane:"database"`
}

type DatabaseConfig struct {
	URL      string `sane:"url"`
	PoolSize int    `sane:"pool_size"`
}

func Init(configFile string) error {
	var conf configuration

	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		return err
	}

	err = sane.Unmarshal(data, &conf)
	if err != nil {
		return err
	}

	Port = conf.Port
	Env = conf.Env
	Database = conf.Database
	// TODO
	DisposableEmailDomains = map[string]bool{}

	return nil
}
