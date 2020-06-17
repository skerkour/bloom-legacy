package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) UpdateCustomer(ctx context.Context, db db.Queryer, customer billing.Customer) (err error) {
	return
}
