package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) FindCustomerByStripeCustomerID(ctx context.Context, db db.Queryer, stripeCustomerID string) (ret billing.Customer, err error) {
	return
}
