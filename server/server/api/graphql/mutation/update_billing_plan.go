package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
)

// UpdateBillingPlan is used by admons to update a plan
func (r *Resolver) UpdateBillingPlan(ctx context.Context, input model.BillingPlanInput) (ret *model.BillingPlan, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if !currentUser.IsAdmin {
		return ret, gqlerrors.AdminRoleRequired()
	}

	params := billing.UpdatePlanParams{
		ID:          input.ID,
		Name:        input.Name,
		Product:     input.Product.String(),
		StripeID:    input.StripeID,
		Description: input.Description,
		IsPublic:    input.IsPublic,
		Storage:     input.Storage,
	}
	plan, err := billing.UpdatePlan(ctx, currentUser, params)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.BillingPlan{
		ID:          plan.ID,
		Name:        plan.Name,
		Description: plan.Description,
		Product:     model.BillingProduct(plan.Product),
		Price:       plan.Price,
		IsPublic:    plan.IsPublic,
		Storage:     plan.Storage,
		StripeID:    &plan.StripeID,
	}
	return ret, nil
}
