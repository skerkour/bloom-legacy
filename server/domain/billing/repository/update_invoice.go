package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) UpdateInvoice(ctx context.Context, db db.Queryer, invoice billing.Invoice) (err error) {
	return
}
