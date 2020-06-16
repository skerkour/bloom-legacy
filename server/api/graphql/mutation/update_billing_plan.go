package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

// UpdateBillingPlan is used by admons to update a plan
func (resolver *Resolver) UpdateBillingPlan(ctx context.Context, input model.BillingPlanInput) (ret *model.BillingPlan, err error) {
	params := billing.UpdatePlanParams{
		ID:          input.ID,
		Name:        input.Name,
		Product:     input.Product.String(),
		StripeID:    input.StripeID,
		Description: input.Description,
		IsPublic:    input.IsPublic,
		Storage:     input.Storage,
	}
	plan, err := resolver.billingService.UpdatePlan(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
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
	return
}
