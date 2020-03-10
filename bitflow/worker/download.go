package worker

import (
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/hex"
	"encoding/json"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/anacrolix/torrent"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/s3"
	"github.com/aws/aws-sdk-go/service/sqs"
	"github.com/satori/go.uuid"
	"gitlab.com/bloom42/bloom/bitflow/shared/async"
	"gitlab.com/bloom42/bloom/bitflow/shared/bitflow"
	"gitlab.com/bloom42/libs/rz-go"
	"gitlab.com/bloom42/libs/rz-go/log"
)

func generateUUID() (string, error) {
	uuidV4 := uuid.NewV4()

	return uuidV4.String(), nil
}

func (worker *Worker) download(id, url string) {
	// uplaod all to S3
	s3Svc := s3.New(worker.awsSession)
	sqsSvc := sqs.New(worker.awsSession)

	log.Info("request received", rz.String("id", id), rz.String("url", url))
	config := torrent.NewDefaultClientConfig()
	config.DataDir = filepath.Join("downloads", id)
	c, _ := torrent.NewClient(config)
	defer c.Close()
	t, _ := c.AddMagnet(url)
	<-t.GotInfo()
	t.DownloadAll()

	asyncMessage1 := async.Message{
		Type: "download_started",
		Data: bitflow.DownloadStartedMessage{ID: id, Name: t.Name()},
	}

	messageData1, err := json.Marshal(asyncMessage1)
	if err != nil {
		log.Error("marshaling download_started message", rz.Err(err))
		return
	}

	qURL := worker.config.AWSSQSBitflowToAPI
	_, err = sqsSvc.SendMessage(&sqs.SendMessageInput{
		DelaySeconds: aws.Int64(0),
		MessageBody:  aws.String(string(messageData1)),
		QueueUrl:     &qURL,
	})

	go func() {
		// send progress updated message
		var progress int32
		length := t.Length()

		for {
			time.Sleep(2 * time.Second)
			completed := t.BytesCompleted()
			if completed == length {
				return
			}
			progress = int32(float64(completed) / float64(length) * 100)
			if progress != progress {
				asyncMessage1 := async.Message{
					Type: "download_progress_updated",
					Data: bitflow.ProgressUpdatedMessage{ID: id, Progress: progress},
				}

				messageData1, err := json.Marshal(asyncMessage1)
				if err != nil {
					log.Error("marshaling download_progress_updated message", rz.Err(err))
					return
				}

				_, err = sqsSvc.SendMessage(&sqs.SendMessageInput{
					DelaySeconds: aws.Int64(0),
					MessageBody:  aws.String(string(messageData1)),
					QueueUrl:     &qURL,
				})
			}
		}
	}()

	c.WaitAll()
	log.Info("torrent downloaded")

	result := bitflow.DownloadCompletedMessage{
		DownloadID: id,
		Files:      []bitflow.DownloadedFile{},
	}

	for _, torrentFile := range t.Files() {
		var err error
		torrentFilePath := torrentFile.Path()
		filePath := filepath.Join("downloads", id, torrentFilePath)
		file, err := os.Open(filePath)
		if err != nil {
			log.Error("openeing file", rz.String("file", filePath), rz.Err(err))
			continue
		}

		// md5
		h := md5.New()
		if _, err := io.Copy(h, file); err != nil {
			log.Error("hashing md5:", rz.String("file", filePath), rz.Err(err))
			continue
		}
		file.Seek(0, 0)
		md5Hash := hex.EncodeToString(h.Sum(nil))

		//sha1
		h = sha1.New()
		if _, err := io.Copy(h, file); err != nil {
			log.Error("hashing sha1:", rz.String("file", filePath), rz.Err(err))
			continue
		}
		file.Seek(0, 0)
		sha1Hash := hex.EncodeToString(h.Sum(nil))

		// sha256
		h = sha256.New()
		if _, err := io.Copy(h, file); err != nil {
			log.Error("hashing sha256:", rz.String("file", filePath), rz.Err(err))
			continue
		}
		file.Seek(0, 0)
		sha256Hash := hex.EncodeToString(h.Sum(nil))

		// sha512
		h = sha512.New()
		if _, err := io.Copy(h, file); err != nil {
			log.Error("hashing sha512:", rz.String("file", filePath), rz.Err(err))
			continue
		}
		file.Seek(0, 0)
		sha512Hash := hex.EncodeToString(h.Sum(nil))

		info, err := file.Stat()
		if _, err := io.Copy(h, file); err != nil {
			log.Error("retrieving file's metadara", rz.String("file", filePath), rz.Err(err))
			continue
		}

		buffer := make([]byte, info.Size())
		file.Seek(0, 0)
		_, err = file.Read(buffer)
		if err != nil {
			log.Error("reading file in buffer", rz.String("file", filePath), rz.Err(err))
			continue
		}
		contentType := http.DetectContentType(buffer)
		file.Seek(0, 0)

		fileUUID, err := generateUUID()
		if err != nil {
			log.Error("generating file's uuid", rz.Err(err))
			return
		}
		fileID := filepath.Join(result.DownloadID, fileUUID)
		path := filepath.Join("/", "bitflow", fileID)
		url := "https://s3." + worker.config.AWSRegion + ".amazonaws.com/" + worker.config.AWSS3Bucket + path

		downloadedFile := bitflow.DownloadedFile{
			ID:     fileID,
			Name:   torrentFile.DisplayPath(),
			Path:   torrentFilePath,
			Type:   contentType,
			Size:   uint64(info.Size()),
			URL:    url,
			MD5:    md5Hash,
			SHA1:   sha1Hash,
			SHA256: sha256Hash,
			SHA512: sha512Hash,
		}
		result.Files = append(result.Files, downloadedFile)

		_, err = s3Svc.PutObject(&s3.PutObjectInput{
			Bucket: aws.String(worker.config.AWSS3Bucket),
			Key:    aws.String(path),
			Body:   file,
		})

		if err != nil {
			log.Error("uploading to file S3", rz.String("file", filePath), rz.Err(err))
		} else {
			log.Info("file successfully uploaded to S3", rz.String("file", filePath))
		}
		file.Close()
	}
	// send result to sqs

	asyncMessage := async.Message{
		Type: "download_completed",
		Data: result,
	}

	messageData, err := json.Marshal(asyncMessage)
	if err != nil {
		log.Error("marshaling result queue message", rz.Err(err))
		return
	}

	_, err = sqsSvc.SendMessage(&sqs.SendMessageInput{
		DelaySeconds: aws.Int64(0),
		MessageBody:  aws.String(string(messageData)),
		QueueUrl:     &qURL,
	})

	// remove folder
	os.RemoveAll(id)
}
