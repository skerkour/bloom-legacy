package worker

import (
	"encoding/json"
	"os"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/credentials"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/sqs"
	"github.com/getsentry/sentry-go"
	"gitlab.com/bloom42/bloom/bitflow/shared/async"
	"gitlab.com/bloom42/bloom/bitflow/shared/bitflow"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

type Worker struct {
	awsSession *session.Session
	config     config
}

func (worker *Worker) init() error {
	err := worker.initConfig()
	if err != nil {
		return err
	}

	err = sentry.Init(sentry.ClientOptions{
		Dsn:         os.Getenv("SENTRY_URL"),
		Environment: worker.config.GoEnv,
	})
	if err != nil {
		return err
	}

	awsConf := aws.Config{
		Credentials: credentials.NewStaticCredentials(
			worker.config.AWSAccessKeyID,
			worker.config.AWSSecretAccessKey,
			"",
		),
	}
	awsConf.Region = aws.String(worker.config.AWSRegion)
	worker.awsSession = session.New(&awsConf)
	return nil
}

func (worker *Worker) Run() error {
	var err error

	err = worker.init()
	if err != nil {
		return err
	}

	sqsService := sqs.New(worker.awsSession)

	qURL := worker.config.AWSSQSAPIToBitflow
	log.Info("listenning queue for async messages", rz.String("queue", qURL))
	for {
		result, err := sqsService.ReceiveMessage(&sqs.ReceiveMessageInput{
			AttributeNames: []*string{
				aws.String(sqs.MessageSystemAttributeNameSentTimestamp),
			},
			MessageAttributeNames: []*string{
				aws.String(sqs.QueueAttributeNameAll),
			},
			QueueUrl:            &qURL,
			MaxNumberOfMessages: aws.Int64(1),
		})

		if err != nil {
			log.Error("error receiving SQS message", rz.Err(err))
			continue
		}

		log.Info("sqs request ended", rz.Int("messages", len(result.Messages)))

		if len(result.Messages) == 0 {
			continue
		}

		for _, message := range result.Messages {

			asyncMessage := async.DecodedMessage{}
			err := json.Unmarshal([]byte(*message.Body), &asyncMessage)
			if err != nil {
				log.Error("error decoding async message", rz.Err(err))
				continue
			}

			switch asyncMessage.Type {
			case "download_queued":
				dlMessage := bitflow.DownloadQueuedMessage{}
				err := json.Unmarshal(asyncMessage.Data, &dlMessage)
				if err != nil {
					log.Error("unmarshalling asyn message", rz.Err(err))
					continue
				}
				go worker.download(dlMessage.ID, dlMessage.URL)
			}

			_, err = sqsService.DeleteMessage(&sqs.DeleteMessageInput{
				QueueUrl:      &qURL,
				ReceiptHandle: message.ReceiptHandle,
			})

			if err != nil {
				log.Error("error deleting message from SQS queue", rz.Err(err))
				continue
			}
		}

	}
}
