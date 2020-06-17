package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

// CreateBillingPlan is used by instance's admin to create a plan
func (resolver *Resolver) CreateBillingPlan(ctx context.Context, input model.BillingPlanInput) (ret *model.BillingPlan, err error) {
	params := billing.CreatePlanParams{
		Name:        input.Name,
		StripeID:    input.StripeID,
		Description: input.Description,
		Product:     input.Product.String(),
		Storage:     input.Storage,
	}
	newPlan, err := resolver.billingService.CreatePlan(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.BillingPlan{
		ID:          newPlan.ID,
		Name:        newPlan.Name,
		Description: newPlan.Description,
		Product:     model.BillingProduct(newPlan.Product),
		Price:       newPlan.Price,
		Storage:     newPlan.Storage,
		StripeID:    &newPlan.StripeID,
	}
	return
}
