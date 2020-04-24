package model

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/billing"
	"gitlab.com/bloom42/lily/uuid"
)

type BillingPlanResolver struct{}

type BillingPlan struct {
	ID          uuid.UUID      `json:"id"`
	Price       int64          `json:"price"`
	Name        string         `json:"name"`
	Description string         `json:"description"`
	IsPublic    bool           `json:"isPublic"`
	Product     BillingProduct `json:"product"`
	Storage     int64          `json:"storage"`
	StripeID    *string        `json:"stripeId"`
}

func (resolver *BillingPlanResolver) Subscribers(ctx context.Context, plan *BillingPlan) (*UserConnection, error) {
	var ret *UserConnection
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	count, err := billing.GetSubscribersCountForPlanId(ctx, uuid.UUID(plan.ID))
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &UserConnection{
		TotalCount: count,
	}
	return ret, nil
}
