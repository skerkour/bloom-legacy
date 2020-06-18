package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) FindInvoiceByStripeInvoiceID(ctx context.Context, db db.Queryer, stripeInvoiceID string) (ret billing.Invoice, err error) {
	return
}
