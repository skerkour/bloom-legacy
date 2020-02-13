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

	plan, err := billing.FindPlanById(ctx, tx, *input.ID)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	plan, err = billing.UpdatePlan(ctx, tx, currentUser, plan, input.Name, input.StripeID,
		input.Description, input.Product.String(), input.IsPublic, int64(input.Storage))
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

	var stripeId *string
	if currentUser.IsAdmin {
		stripeId = &plan.StripeID
	}
	ret = &model.BillingPlan{
		ID:          plan.ID,
		Name:        plan.Name,
		Description: plan.Description,
		Product:     model.BillingProduct(plan.Product),
		Price:       model.Int64(plan.Price),
		IsPublic:    plan.IsPublic,
		Storage:     model.Int64(plan.Storage),
		StripeID:    stripeId,
	}
	return ret, nil
}
