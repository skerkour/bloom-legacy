package worker

import (
	"fmt"
	"os"

	// "github.com/astrolib/godotenv"
	"gitlab.com/bloom42/bloom/bitflow/version"
	// "github.com/bloom42/denv-go"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

type config struct {
	GoEnv              string
	AWSSecretAccessKey string
	AWSAccessKeyID     string
	AWSRegion          string
	AWSSQSAPIToBitflow string
	AWSSQSBitflowToAPI string
	AWSS3Bucket        string
	SentryURL          string
}

// RequiredEnvVars are the required environment variables to run the server
var RequiredEnvVars = []string{
	"AWS_SECRET_ACCESS_KEY",
	"AWS_ACCESS_KEY_ID",
	"AWS_REGION",
	"AWS_SQS_BITFLOW_TO_API",
	"AWS_SQS_API_TO_BITFLOW",
	"AWS_S3_BUCKET",
	"SENTRY_URL",
}

var DefaultEnv = map[string]string{
	"GO_ENV": "development",
}

// CheckEnv checks if the required environment variables are present
func checkEnv() {
	for _, v := range RequiredEnvVars {
		if os.Getenv(v) == "" {
			panic(fmt.Sprintf("Missing environment variable: %s", v))
		}
	}
}

// Init loads the server configuration
func (worker *Worker) initConfig() error {
	// godotenv.Load()
	checkEnv()
	// denv.Init(denv.Env{
	// 	"GO_ENV": "development",
	// })
	var conf config

	conf.GoEnv = os.Getenv("GO_ENV")
	conf.AWSRegion = os.Getenv("AWS_REGION")
	conf.AWSAccessKeyID = os.Getenv("AWS_ACCESS_KEY_ID")
	conf.AWSSecretAccessKey = os.Getenv("AWS_SECRET_ACCESS_KEY")
	conf.AWSSQSAPIToBitflow = os.Getenv("AWS_SQS_API_TO_BITFLOW")
	conf.AWSSQSBitflowToAPI = os.Getenv("AWS_SQS_BITFLOW_TO_API")
	conf.AWSS3Bucket = os.Getenv("AWS_S3_BUCKET")
	conf.SentryURL = os.Getenv("SENTRY_URL")

	// configure logger
	if conf.GoEnv == "production" {
		log.SetLogger(log.With(rz.Level(rz.InfoLevel)))
	} else {
		log.SetLogger(log.With(rz.Formatter(rz.FormatterConsole())))
	}

	hostname, _ := os.Hostname()
	log.SetLogger(log.With(
		rz.Fields(
			rz.Dict("service", log.NewDict(
				rz.String("name", version.Name),
				rz.String("version", version.Version),
			)),
			rz.String("host", hostname),
			rz.String("environment", conf.GoEnv),
		),
	))

	log.Info("worker configuration successfully loaded",
		rz.String("aws_region", conf.AWSRegion),
		rz.String("AWS_SQS_API_TO_BITFLOW", conf.AWSSQSAPIToBitflow),
		rz.String("AWS_SQS_BITFLOW_TO_API", conf.AWSSQSBitflowToAPI),
		rz.String("aws_s3_bucket", conf.AWSS3Bucket),
	)

	worker.config = conf
	return nil
}
