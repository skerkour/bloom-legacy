package notification

import (
	"bytes"
	"net/mail"
	"text/template"

	"gitlab.com/bloom42/lily/email"
)

// DefaultTemplateParams are the params for SendHTMLEmailWithDefaultTemplate
type DefaultTemplateParams struct {
	Title string
	Body  string
}

// SendHTMLEmailWithDefaultTemplate sends an HTML message with the default HTML template
func SendHTMLEmailWithDefaultTemplate(from *mail.Address, to []*mail.Address, subject string, body []byte) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("DefaultEmailTemplate").Parse(DEFAULT_EMAIL_TEMPLATE))

	params := DefaultTemplateParams{
		Title: subject,
		Body:  string(body),
	}
	err := tmpl.Execute(&content, params)
	if err != nil {
		return err
	}

	message := email.Email{
		From:    from,
		To:      to,
		Subject: subject,
		HTML:    content.Bytes(),
		// Text:    markdown,
	}
	return email.Send(message)
}
