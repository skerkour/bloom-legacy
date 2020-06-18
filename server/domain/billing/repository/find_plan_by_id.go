package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindPlanByID(ctx context.Context, db db.Queryer, planID uuid.UUID) (ret billing.Plan, err error) {
	query := "SELECT * FROM billing_plans WHERE id = $1"

	err = db.Get(ctx, &ret, query, planID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Plan not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindPlanByID: finding plan"
			logger.Error(errMessage, log.Err("error", err), log.UUID("plan.id", planID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
