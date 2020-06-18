package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) UpdatePlan(ctx context.Context, db db.Queryer, plan billing.Plan) (err error) {
	query := `UPDATE billing_plans SET
		updated_at = $1, name = $2, stripe_id = $3, price = $4, description = $5, product = $6, storage = $7
		WHERE id = $8`

	_, err = db.Exec(ctx, query, plan.UpdatedAt, plan.Name, plan.StripeID, plan.Price, plan.Description,
		plan.Product, plan.Storage, plan.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.UpdatePlan: updating plan"
		logger.Error(errMessage, log.Err("error", err), log.UUID("plan.id", plan.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
