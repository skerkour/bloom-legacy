package billing

import (
	"bytes"
	"net/mail"
	"text/template"

	"gitlab.com/bloom42/lily/email"
)

const paymentFailedEmailTemplate = `
Unfortunately, your most recent invoice payment for {{ .Amount }} was declined.
  This could be due to a change in your card number, your card expiring,
  cancellation of your credit card, or the card issuer not recognizing the
  payment and therefore taking action to prevent it.

  Please update your payment information as soon as possible
`

type paymentFailedEmailData struct {
	Amount float64
}

func SendPaymentFailedEmail(toAddr string, amount int64) error {
	var content bytes.Buffer
	tmpl := template.Must(template.New("paymentFailedEmailTemaplte").Parse(paymentFailedEmailTemplate))
	data := paymentFailedEmailData{
		Amount: float64(amount) / 100,
	}

	subject := "Your most recent invoice payment failed"
	err := tmpl.Execute(&content, data)
	if err != nil {
		return err
	}

	message := email.Email{
		From:    &mail.Address{Name: "Bloom", Address: "hello@bloom.sh"},
		To:      []*mail.Address{&mail.Address{Name: "", Address: toAddr}},
		Subject: subject,
		HTML:    content.Bytes(),
	}
	return email.Send(message)
}
