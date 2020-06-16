package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// DeleteBillingPlan is used by an instance's admin to delete a plan
func (resolver *Resolver) DeleteBillingPlan(ctx context.Context, input model.DeleteBillingPlanInput) (ret bool, err error) {
	err = resolver.billingService.DeletePlan(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return ret, nil
}
