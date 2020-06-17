package service

import (
	"context"

	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *BillingService) PaymentFailed(ctx context.Context, stripeInvoice stripe.Invoice) (err error) {
	logger := log.FromCtx(ctx)

	if stripeInvoice.ID == "" {
		logger.Error("billing.PaymentFailed: stripeInvoice is null")
		err = errors.Internal("", nil)
		return
	}

	customer, err := service.billingRepo.FindCustomerByStripeCustomerID(ctx, service.db, stripeInvoice.Customer.ID)
	if err != nil {
		return
	}

	go service.sendPaymentFailedEmail(ctx, customer.Email, stripeInvoice.AmountDue)
	return
}
