package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) GetSubscribersCountForPlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (ret int64, err error) {
	query := "SELECT COUNT(*) FROM billing_customers WHERE plan_id = $1"

	err = db.Get(ctx, &ret, query, planID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.GetSubscribersCountForPlan: getting subscribers count"
		logger.Error(errMessage, log.Err("error", err), log.UUID("plan.id", planID))
		err = errors.Internal(errMessage, err)
	}
	return
}
