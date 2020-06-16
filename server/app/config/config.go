package config

import (
	"encoding/json"
	"io/ioutil"
	"os"
	"regexp"
	"strings"
)

// Config contains all the configuration of the server
type Config struct {
	Env        string         `json:"env"`
	WebsiteURL string         `json:"website_url"`
	HTTP       HTTPConfig     `json:"http"`
	Database   DatabaseConfig `json:"database"`
	SMTP       SMTPConfig     `json:"smtp"`
	Stripe     StripeConfig   `json:"stripe"`
	Sentry     SentryConfig   `json:"sentry"`
}

// DatabaseConfig contains the data necessary to connect to a database
type DatabaseConfig struct {
	URL      string `json:"url"`
	PoolSize int    `json:"pool_size"`
}

// SMTPConfig contains the data necessary to send emails using the SMTP protocol
type SMTPConfig struct {
	Port     uint16 `json:"port"`
	Host     string `json:"host"`
	Username string `json:"username"`
	Password string `json:"password"`
}

// HTTPConfig contains the data specific to the HTTP(s) server
type HTTPConfig struct {
	Port           uint16   `json:"port"`
	HTTPSPort      *uint16  `json:"https_port"`
	Domains        []string `json:"domains"`
	CertsDirectory string   `json:"certs_directory"`
	CertsEmail     string   `json:"certs_email"`
}

// StripeConfig contains the data to connect to Stripe
type StripeConfig struct {
	SecretKey     string `json:"secret_key"`
	PublicKey     string `json:"public_key"`
	WebhookSecret string `json:"webhook_secret"`
}

// SentryConfig contains the data to report errors and crashes to Sentry
type SentryConfig struct {
	Dsn string `json:"dsn"`
}

// Load and validate the configuration from the given file.
// If an error is found while parsing the file, or validating the data, an error is returned
func Load(configFilePath string) (ret Config, err error) {
	configData, err := ioutil.ReadFile(configFilePath)
	if err != nil {
		return
	}

	configData = []byte(os.ExpandEnv(string(configData)))
	configData = stripComments(configData)

	err = json.Unmarshal(configData, &ret)
	if err != nil {
		return
	}

	return
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

func stripComments(jsonWithComment []byte) []byte {
	re := regexp.MustCompile(`(?m)(\/\/[^"\n]+$)|(^\s+\/\/.*$)`)
	return re.ReplaceAll(jsonWithComment, []byte(""))
}
