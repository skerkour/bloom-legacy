package notification

import (
	"fmt"
	"gitlab.com/bloom42/bloom/server/config"
	"net/smtp"
)

func Init() error {
	auth := smtp.PlainAuth("", config.SMTP.Username, config.SMTP.Password, config.SMTP.Host)
	emailer = mailer{
		auth:    auth,
		address: fmt.Sprintf("%s:%d", config.SMTP.Host, config.SMTP.Port),
	}
	return nil
}
