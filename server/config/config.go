package config

import (
	"io/ioutil"

	"gitlab.com/bloom42/libs/sane-go"
)

var DisposableEmailDomains map[string]bool
var Env string
var Server ServerConfig
var Database DatabaseConfig
var SMTP SMTPConfig
var Stripe StripeConfig
var Sentry SentryConfig

type configuration struct {
	Env      string         `sane:"env"`
	Server   ServerConfig   `sane:"server"`
	Database DatabaseConfig `sane:"database"`
	SMTP     SMTPConfig     `sane:"smtp"`
	Stripe   StripeConfig   `sane:"stripe"`
	Sentry   SentryConfig   `sane:"sentry"`
}

type DatabaseConfig struct {
	URL      string `sane:"url"`
	PoolSize int    `sane:"pool_size"`
}

type SMTPConfig struct {
	Port     uint16 `sane:"port"`
	Host     string `sane:"host"`
	Username string `sane:"username"`
	Password string `sane:"password"`
}

type ServerConfig struct {
	Port uint16 `sane:"port"`
}

type StripeConfig struct {
	SecretKey string `sane:"secret_key"`
	PublicKey string `sane:"public_key"`
}

type SentryConfig struct {
	Dsn string `sane:"dsn"`
}

func Init(configFile string) error {
	var parsedConfig configuration

	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		return err
	}

	err = sane.Unmarshal(data, &parsedConfig)
	if err != nil {
		return err
	}

	Env = parsedConfig.Env
	Server = parsedConfig.Server
	Database = parsedConfig.Database
	SMTP = parsedConfig.SMTP
	Stripe = parsedConfig.Stripe
	Sentry = parsedConfig.Sentry
	// TODO
	DisposableEmailDomains = map[string]bool{}

	return nil
}
