package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) FindAllPlans(ctx context.Context, db db.Queryer) (ret []billing.Plan, err error) {
	ret = []billing.Plan{}
	query := "SELECT * FROM billing_plans"

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.FindAllPlans: finding plans"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
	}
	return
}
