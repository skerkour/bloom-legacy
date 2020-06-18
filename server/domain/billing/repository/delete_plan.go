package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) DeletePlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (err error) {
	query := "DELETE FROM billing_plans WHERE id = $1"

	_, err = db.Exec(ctx, query, planID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.DeletePlan: deleting plan"
		logger.Error(errMessage, log.Err("error", err), log.UUID("plan.id", planID))
		err = errors.Internal(errMessage, err)
	}
	return
}
