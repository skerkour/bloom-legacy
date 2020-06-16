package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/billing"
)

// DeleteBillingPlan is used by an instance's admin to delete a plan
func (r *Resolver) DeleteBillingPlan(ctx context.Context, input model.DeleteBillingPlanInput) (bool, error) {
	ret := false
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	err := billing.DeletePlan(ctx, currentUser, input.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = true
	return ret, nil
}
