package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) FindDefaultPlan(ctx context.Context, db db.Queryer) (ret billing.Plan, err error) {
	return
}
