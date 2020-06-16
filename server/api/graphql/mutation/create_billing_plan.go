package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/billing"
)

// CreateBillingPlan is used by instance's admin to create a plan
func (r *Resolver) CreateBillingPlan(ctx context.Context, input model.BillingPlanInput) (ret *model.BillingPlan, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := billing.CreatePlanParams{
		Name:        input.Name,
		StripeID:    input.StripeID,
		Description: input.Description,
		Product:     input.Product.String(),
		Storage:     input.Storage,
		IsPublic:    input.IsPublic,
	}
	newPlan, err := billing.CreatePlan(ctx, currentUser, params)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.BillingPlan{
		ID:          newPlan.ID,
		Name:        newPlan.Name,
		Description: newPlan.Description,
		Product:     model.BillingProduct(newPlan.Product),
		Price:       newPlan.Price,
		IsPublic:    newPlan.IsPublic,
		Storage:     newPlan.Storage,
		StripeID:    &newPlan.StripeID,
	}
	return ret, nil
}
