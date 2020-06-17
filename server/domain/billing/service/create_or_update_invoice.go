package service

import (
	"context"

	"github.com/stripe/stripe-go"
)

func (service *BillingService) CreateOrUpdateInvoice(ctx context.Context, invoice stripe.Invoice) (err error) {
	return
}
