package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
)

// BillingPlans find all visible billing plans for current user
func (resolver *Resolver) BillingPlans(ctx context.Context) (ret *model.BillingPlanConnection, err error) {
	me, err := resolver.usersService.Me(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	plans, err := resolver.billingService.FindPlans(ctx)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.BillingPlanConnection{
		Nodes:      []*model.BillingPlan{},
		TotalCount: int64(len(plans)),
	}

	for _, plan := range plans {
		var stripeID *string
		if me.IsAdmin {
			stripeID = &plan.StripeID
		}
		billingPlan := &model.BillingPlan{
			ID:          plan.ID,
			Name:        plan.Name,
			Description: plan.Description,
			Price:       plan.Price,
			IsPublic:    plan.IsPublic,
			Product:     model.BillingProduct(plan.Product),
			Storage:     plan.Storage,
			StripeID:    stripeID,
		}
		ret.Nodes = append(ret.Nodes, billingPlan)
	}
	return
}
