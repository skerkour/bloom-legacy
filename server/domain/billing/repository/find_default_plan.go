package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) FindDefaultPlan(ctx context.Context, db db.Queryer) (ret billing.Plan, err error) {
	query := "SELECT * FROM billing_plans WHERE product = $1"

	err = db.Get(ctx, &ret, query, billing.ProductFree)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Plan not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindDefaultPlan: finding plan"
			logger.Error(errMessage, log.Err("error", err), log.String("plan.product", billing.ProductFree))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
