package model

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

// BillingPlanResolver is the resolver for the BillingPlan type
type BillingPlanResolver struct{}

func NewBillingPlanResolver() *BillingPlanResolver {
	return &BillingPlanResolver{}
}

// BillingPlan represents a plan
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

// Subscribers is used by admin to get the subscribers of a plan
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
