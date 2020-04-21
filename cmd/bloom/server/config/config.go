package config

import (
	"encoding/json"
	"io/ioutil"
	"os"
	"regexp"
	"strings"
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
	Env        string         `json:"env"`
	Server     ServerConfig   `json:"server"`
	Database   DatabaseConfig `json:"database"`
	SMTP       SMTPConfig     `json:"smtp"`
	Stripe     StripeConfig   `json:"stripe"`
	Sentry     SentryConfig   `json:"sentry"`
	WebsiteUrl string         `json:"website_url"`
	Bitflow    BitflowConfig  `json:"bitflow"`
}

type DatabaseConfig struct {
	URL      string `json:"url"`
	PoolSize int    `json:"pool_size"`
}

type SMTPConfig struct {
	Port     uint16 `json:"port"`
	Host     string `json:"host"`
	Username string `json:"username"`
	Password string `json:"password"`
}

type ServerConfig struct {
	Port           uint16   `json:"port"`
	Domains        []string `json:"domains"`
	HTTP           bool     `json:"http"`
	CertsDirectory string   `json:"certs_directory"`
	CertsEmail     string   `json:"certs_email"`
}

type StripeConfig struct {
	SecretKey     string `json:"secret_key"`
	PublicKey     string `json:"public_key"`
	WebhookSecret string `json:"webhook_secret"`
}

type SentryConfig struct {
	Dsn string `json:"dsn"`
}

type BitflowConfig struct {
	Secret string `json:"secret"`
}

func Init(configFile string) error {
	var parsedConfig configuration

	data, err := ioutil.ReadFile(configFile)
	if err != nil {
		return err
	}

	configFileDataStr := replaceEnvVars(string(data))
	configFileDataStr = stripeComments(configFileDataStr)

	err = json.Unmarshal([]byte(configFileDataStr), &parsedConfig)
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

func stripeComments(jsonWithComment string) string {
	re := regexp.MustCompile(`(?m)(\/\/[^"\n]+$)|(^\s+\/\/.*$)`)
	return re.ReplaceAllString(jsonWithComment, "")
}
