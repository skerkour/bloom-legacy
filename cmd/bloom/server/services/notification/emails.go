package notification

import (
	"bytes"
	"net/mail"
	"text/template"

	"gitlab.com/bloom42/lily/email"
)

type DefaultTemplateParams struct {
	Title           string
	OnlineLink      string
	Content         string
	UnsubscribeLink string
}

func SendHTMLEmailWithDefaultTemplate(from *mail.Address, to *mail.Address, subject string, markdown []byte, params DefaultTemplateParams) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("DefaultEmailTemplate").Parse(DefaultEmailTemplate))

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
