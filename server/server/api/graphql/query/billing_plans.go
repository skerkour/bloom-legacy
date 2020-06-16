package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
)

// BillingPlans find all visible billing plans for current user
func (r *Resolver) BillingPlans(ctx context.Context) (ret *model.BillingPlanConnection, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	plans, err := billing.FindPlans(ctx, currentUser)
	if err != nil {
		err = gqlerrors.New(err)
		return
	}

	ret = &model.BillingPlanConnection{
		Nodes:      []*model.BillingPlan{},
		TotalCount: int64(len(plans)),
	}

	for _, plan := range plans {
		var stripeID *string
		if currentUser.IsAdmin {
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

	return ret, nil
}
