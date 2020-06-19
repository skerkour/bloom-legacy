package service

import (
	"bytes"
	"context"
	"html/template"
	"net/mail"

	"gitlab.com/bloom42/bloom/server/app/config"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/email"
	"gitlab.com/bloom42/gobox/log"
)

func allowedStorageForProduct(product string) int64 {
	if product == billing.ProductUltra {
		return billing.StorageUltra
	} else if product == billing.ProductLite {
		return billing.StorageLite
	} else if product == billing.ProductPro {
		return billing.StoragePro
	} else {
		return billing.StorageFree
	}
}

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

func (service *BillingService) sendPaymentFailedEmail(ctx context.Context, toAddr string, amount int64) (err error) {
	logger := log.FromCtx(ctx)
	var content bytes.Buffer
	tmpl := template.Must(template.New("paymentFailedEmailTemaplte").Parse(paymentFailedEmailTemplate))
	data := paymentFailedEmailData{
		Amount: float64(amount) / 100,
	}

	subject := "Your most recent invoice payment failed"
	err = tmpl.Execute(&content, data)
	if err != nil {
		errMessage := "billing.sendPaymentFailedEmail: executing template"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	message := email.Email{
		From:    config.DefaultEmailAddressFrom,
		To:      []mail.Address{{Name: "", Address: toAddr}},
		Subject: subject,
		HTML:    content.Bytes(),
		Text:    content.Bytes(),
	}
	err = service.mailer.Send(ctx, message)
	if err != nil {
		errMessage := "billing.sendPaymentFailedEmail: Sending email"
		logger.Error(errMessage, log.Err("err", err))
		err = errors.Internal(errMessage, err)
	}
	return
}
