package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

// BillingPlans find all visible billing plans for current user
func (r *Resolver) BillingPlans(ctx context.Context) (*model.BillingPlanConnection, error) {
	var ret *model.BillingPlanConnection
	currentUser := apiutil.UserFromCtx(ctx)

	plans, err := billing.FindPlans(ctx, currentUser)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.BillingPlanConnection{
		Edges:      []*model.BillingPlanEdge{},
		TotalCount: model.Int64(len(plans)),
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
			Price:       model.Int64(plan.Price),
			IsPublic:    plan.IsPublic,
			Product:     model.BillingProduct(plan.Product),
			Storage:     model.Int64(plan.Storage),
			StripeID:    stripeID,
		}
		edge := &model.BillingPlanEdge{
			Node: billingPlan,
		}
		ret.Edges = append(ret.Edges, edge)
	}

	return ret, nil
}
