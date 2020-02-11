package model

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

type BillingPlanResolver struct{}

type BillingPlan struct {
	ID          string          `json:"id"`
	Price       Int64           `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	IsPublic    bool            `json:"isPublic"`
	Tier        BillingPlanTier `json:"tier"`
	Storage     Int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
}

func (resolver *BillingPlanResolver) Subscribers(ctx context.Context, plan *BillingPlan) (*Int64, error) {
	var ret *Int64
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil || !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	count, err := billing.GetSubscribersCountForPlanId(ctx, plan.ID)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	countInt64 := Int64(count)
	ret = &countInt64
	return ret, nil
}
