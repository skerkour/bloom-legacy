package smtp

import (
	"bytes"
	"context"
	"text/template"

	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/driver"
	"gitlab.com/bloom42/gobox/email"
)

// Mailer implements the `email.Mailer` interface to send emails using SMTP
type Mailer struct {
	mailer          email.Mailer
	defaultTemplate *template.Template
}

// NewMailer returns a new smtp Mailer
func NewMailer(config config.SMTPConfig) *Mailer {
	conf := email.SMTPConfig{
		Host:     config.Host,
		Port:     config.Port,
		Username: config.Username,
		Password: config.Password,
	}
	defaultTemplate := template.Must(template.New("defaultEmailTemplate").Parse(driver.DefaultEmailTemplate))

	return &Mailer{
		mailer:          email.NewMailer(conf),
		defaultTemplate: defaultTemplate,
	}
}

type defaultTemplateParams struct {
	Body string
}

// Send an email using the SMTP mailer
func (mailer *Mailer) Send(ctx context.Context, email email.Email) error {
	innerContent := string(email.HTML)
	var content bytes.Buffer

	params := defaultTemplateParams{
		Body: innerContent,
	}

	err := mailer.defaultTemplate.Execute(&content, params)
	if err != nil {
		return err
	}

	email.HTML = content.Bytes()
	return mailer.mailer.Send(email)
}

// SendRaw an email using the SMTP mailer
func (mailer *Mailer) SendRaw(ctx context.Context, email email.Email) error {
	return mailer.mailer.Send(email)
}
