package notification

import (
	"bytes"
	"net/mail"
	"text/template"

	"gitlab.com/bloom42/lily/email"
)

// DefaultTemplateParams are the params for SendHTMLEmailWithDefaultTemplate
type DefaultTemplateParams struct {
	Title           string
	OnlineLink      string
	Content         string
	UnsubscribeLink string
}

// SendHTMLEmailWithDefaultTemplate sends an HTML message with the default HTML template
func SendHTMLEmailWithDefaultTemplate(from *mail.Address, to *mail.Address, subject string, markdown []byte, params DefaultTemplateParams) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("DefaultEmailTemplate").Parse(DEFAULT_EMAIL_TEMPLATE))

	err := tmpl.Execute(&content, params)
	if err != nil {
		return err
	}

	message := email.Email{
		From:    from,
		To:      []*mail.Address{to},
		Subject: subject,
		HTML:    content.Bytes(),
		Text:    markdown,
	}
	return email.Send(message)
}
