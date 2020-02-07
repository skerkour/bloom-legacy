package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) UpdateBillingPlan(ctx context.Context, input model.BillingPlanInput) (*model.BillingPlan, error) {
	var ret *model.BillingPlan
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.UpdateBillingPlan: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorUpdatingPlan))
	}

	if input.ID == nil {
		return ret, gqlerrors.New(errors.New(errors.InvalidArgument, "plan.id should not be null"))
	}
	if input.IsActive == nil {
		return ret, gqlerrors.New(errors.New(errors.InvalidArgument, "plan.isActive should not be null"))
	}

	plan, err := billing.FindPlanById(ctx, tx, *input.ID)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	plan, err = billing.UpdatePlan(ctx, tx, currentUser, plan, input.Name, input.StripeID,
		input.Description, input.Tier.String(), input.Price, *input.IsActive)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.UpdateBillingPlan: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorCreatingPlan))
	}

	ret = &model.BillingPlan{
		ID:          plan.ID,
		Name:        plan.Name,
		Description: plan.Description,
		Tier:        model.BillingPlanTier(plan.Tier),
		Price:       plan.Price,
		IsActive:    plan.IsActive,
	}
	return ret, nil
}
