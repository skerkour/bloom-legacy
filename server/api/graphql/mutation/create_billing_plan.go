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

func (r *Resolver) CreateBillingPlan(ctx context.Context, input model.BillingPlanInput) (*model.BillingPlan, error) {
	var ret *model.BillingPlan
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CreateBillingPlan: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorCreatingPlan))
	}

	newPlan, err := billing.CreatePlan(ctx, tx, currentUser, input.Name, input.StripeID,
		input.Description, input.Product.String(), int64(input.Storage), input.IsPublic)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CreateBillingPlan: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(billing.NewError(billing.ErrorCreatingPlan))
	}

	var stripeId *string
	if currentUser.IsAdmin {
		stripeId = &newPlan.StripeID
	}
	ret = &model.BillingPlan{
		ID:          newPlan.ID,
		Name:        newPlan.Name,
		Description: newPlan.Description,
		Product:     model.BillingProduct(newPlan.Product),
		Price:       model.Int64(newPlan.Price),
		IsPublic:    newPlan.IsPublic,
		Storage:     model.Int64(newPlan.Storage),
		StripeID:    stripeId,
	}
	return ret, nil
}
