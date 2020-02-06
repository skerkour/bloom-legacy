package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) DeleteBillingPlan(ctx context.Context, input model.DeleteBillingPlanInput) (bool, error) {
	ret := false
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.DeleteBillingPlan: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorCreatingPlan))
	}

	err = billing.DeletePlan(ctx, tx, currentUser, input.ID)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.DeleteBillingPlan: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorCreatingPlan))
	}

	ret = true
	return ret, nil
}
