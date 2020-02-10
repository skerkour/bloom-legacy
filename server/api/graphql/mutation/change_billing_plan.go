package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (r *Resolver) ChangeBillingPlan(ctx context.Context, input model.ChangeBillingPlanInput) (*model.BillingPlan, error) {
	var ret *model.BillingPlan
	var err error
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	newPlan, err := billing.ChangePlan(ctx, currentUser, input.UserID, input.GroupID, input.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.BillingPlan{
		ID:          newPlan.ID,
		Name:        newPlan.Name,
		Description: newPlan.Description,
		Tier:        model.BillingPlanTier(newPlan.Tier),
		Price:       newPlan.Price,
		IsActive:    newPlan.IsActive,
		Storage:     model.Int64(newPlan.Storage),
	}
	return ret, nil
}
