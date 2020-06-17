package service

import (
	"context"

	"github.com/stripe/stripe-go"
)

func (service *BillingService) PaymentFailed(ctx context.Context, invoice stripe.Invoice) (err error) {
	return
}
