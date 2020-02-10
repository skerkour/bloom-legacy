package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

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
