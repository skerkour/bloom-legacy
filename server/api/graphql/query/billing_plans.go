package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (r *Resolver) BillingPlans(ctx context.Context) ([]*model.BillingPlan, error) {
	ret := []*model.BillingPlan{}
	currentUser := apiutil.UserFromCtx(ctx)

	plans, err := billing.FindPlans(ctx, currentUser)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	for _, plan := range plans {
		var stripeId *string
		if currentUser.IsAdmin {
			stripeId = &plan.StripeID
		}
		billingPlan := &model.BillingPlan{
			ID:          plan.ID,
			Name:        plan.Name,
			Description: plan.Description,
			Price:       plan.Price,
			IsActive:    plan.IsActive,
			Tier:        model.BillingPlanTier(plan.Tier),
			Storage:     model.Int64(plan.Storage),
			StripeID:    stripeId,
		}
		ret = append(ret, billingPlan)
	}

	return ret, nil
}
