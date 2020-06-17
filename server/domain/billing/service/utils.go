package service

import (
	"bytes"
	"context"
	"html/template"
	"net/mail"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/services/notification"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func allowedStorageForProduct(product string) int64 {
	if product == consts.BILLING_PRODUCT_ULTRA {
		return 100000000000000 // 1TB
	} else if product == consts.BILLING_PRODUCT_LITE {
		return 100000000000 // 100GB
	} else if product == consts.BILLING_PRODUCT_PRO {
		return 400000000000 // 400GB
	} else { // FREE
		return 100000000 // 100MB
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

	from := &mail.Address{Name: notification.DEFAULT_SENDER_NAME, Address: notification.DEFAULT_SENDER_ADDRESS}
	to := []*mail.Address{{Name: "", Address: toAddr}}
	err = notification.SendHTMLEmailWithDefaultTemplate(from, to, subject, content.Bytes())
	if err != nil {
		errMessage := "billing.sendPaymentFailedEmail: sending email"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
