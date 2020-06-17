package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) UpdatePlan(ctx context.Context, db db.Queryer, plan billing.Plan) (err error) {
	return
}
