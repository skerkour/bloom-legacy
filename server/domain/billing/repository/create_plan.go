package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) CreatePlan(ctx context.Context, db db.Queryer, plan billing.Plan) (err error) {
	query := `INSERT INTO billing_plans
	(id, created_at, updated_at, name, description, stripe_id, price, product, storage)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`

	_, err = db.Exec(ctx, query, plan.ID, plan.CreatedAt, plan.UpdatedAt, plan.Name, plan.Description,
		plan.StripeID, plan.Price, plan.Product, plan.Storage)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.CreatePlan: inserting plan"
		logger.Error(errMessage, log.Err("error", err), log.UUID("plan.id", plan.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
