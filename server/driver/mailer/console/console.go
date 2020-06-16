// Package console implements a console Mailer for dev and debugging purpose
package console

import (
	"context"

	"gitlab.com/bloom42/gobox/email"
	"gitlab.com/bloom42/gobox/log"
)

// Mailer implements the `email.Mailer` interface to print email in the console
type Mailer struct {
	logger log.Logger
}

// NewMailer returns a new console Mailer
func NewMailer(logger log.Logger) *Mailer {

	return &Mailer{
		logger: logger,
	}
}

// Send an email using the console mailer
func (mailer *Mailer) Send(ctx context.Context, email email.Email) error {
	mailer.logger.Debug("email",
		log.String("from", email.From.String()),
		log.Any("to", email.To),
		log.String("subject", email.Subject),
		log.String("text", string(email.Text)),
		log.String("html", string(email.HTML)),
		log.Any("headers", email.Headers),
		log.Any("attachments", email.Attachments),
	)
	return nil
}
