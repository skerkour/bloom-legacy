package config

import (
	"io/ioutil"
	"os"
	"regexp"
	"strings"

	"gitlab.com/bloom42/libs/sane-go"
)

var DisposableEmailDomains map[string]bool
var Env string
var Server ServerConfig
var Database DatabaseConfig
var SMTP SMTPConfig
var Stripe StripeConfig
var Sentry SentryConfig
var WebsiteUrl string
var Bitflow BitflowConfig

type configuration struct {
	Env        string         `sane:"env"`
	Server     ServerConfig   `sane:"server"`
	Database   DatabaseConfig `sane:"database"`
	SMTP       SMTPConfig     `sane:"smtp"`
	Stripe     StripeConfig   `sane:"stripe"`
	Sentry     SentryConfig   `sane:"sentry"`
	WebsiteUrl string         `sane:"website_url"`
	Bitflow    BitflowConfig  `sane:"bitflow"`
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
	Port    uint16    `sane:"port"`
	Domains *[]string `sane:"domains"`
}

type StripeConfig struct {
	SecretKey     string `sane:"secret_key"`
	PublicKey     string `sane:"public_key"`
	WebhookSecret string `sane:"webhook_secret"`
}

type SentryConfig struct {
	Dsn string `sane:"dsn"`
}

type BitflowConfig struct {
	Secret string `sane:"secret"`
}

func Init(configFile string) error {
	var parsedConfig configuration

	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		return err
	}

	configFileDataStr := replaceEnvVars(string(data))

	err = sane.Unmarshal([]byte(configFileDataStr), &parsedConfig)
	if err != nil {
		return err
	}

	Env = parsedConfig.Env
	Server = parsedConfig.Server
	Database = parsedConfig.Database
	SMTP = parsedConfig.SMTP
	Stripe = parsedConfig.Stripe
	Sentry = parsedConfig.Sentry
	WebsiteUrl = parsedConfig.WebsiteUrl
	Bitflow = parsedConfig.Bitflow
	// TODO
	DisposableEmailDomains = map[string]bool{}

	return nil
}

func replaceEnvVars(configFileData string) string {
	r := regexp.MustCompile(`\${([^}]*)}`)
	matches := r.FindAllStringSubmatch(configFileData, -1)
	for _, v := range matches {
		envVar := os.Getenv(v[1])
		configFileData = strings.Replace(configFileData, v[0], envVar, 1)
	}
	return configFileData
}
