package smtp

import (
	"context"

	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/gobox/email"
)

// Mailer implements the `email.Mailer` interface to send emails using SMTP
type Mailer struct {
	mailer email.Mailer
}

// NewMailer returns a new smtp Mailer
func NewMailer(config config.SMTPConfig) *Mailer {
	conf := email.SMTPConfig{
		Host:     config.Host,
		Port:     config.Port,
		Username: config.Username,
		Password: config.Password,
	}

	return &Mailer{
		mailer: email.NewMailer(conf),
	}
}

// Send an email using the SMTP mailer
func (mailer *Mailer) Send(ctx context.Context, email email.Email) error {
	return mailer.mailer.Send(email)
}
