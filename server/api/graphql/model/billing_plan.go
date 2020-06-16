package model

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

// BillingPlanResolver is the resolver for the BillingPlan type
type BillingPlanResolver struct {
	billingService billing.Service
}

func NewBillingPlanResolver(billingService billing.Service) *BillingPlanResolver {
	return &BillingPlanResolver{
		billingService: billingService,
	}
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
func (resolver *BillingPlanResolver) Subscribers(ctx context.Context, plan *BillingPlan) (ret *UserConnection, err error) {
	count, err := resolver.billingService.SubscribersCountForPlan(ctx, plan.ID)
	if err != nil {
		return ret, api.NewError(err)
	}

	ret = &UserConnection{
		TotalCount: count,
	}
	return
}
