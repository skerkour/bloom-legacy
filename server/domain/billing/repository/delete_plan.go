package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) DeletePlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (err error) {
	return
}
