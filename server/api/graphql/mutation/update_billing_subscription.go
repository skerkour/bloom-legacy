package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

// UpdateBillingSubscription is used by users to updated their subscriptions
func (resolver *Resolver) UpdateBillingSubscription(ctx context.Context, input model.UpdateBillingSubscriptionInput) (ret *model.BillingSubscription, err error) {
	params := billing.ChangeSubscriptionParams{
		UserID:  input.UserID,
		GroupID: input.GroupID,
		PlanID:  input.PlanID,
	}
	customer, newPlan, err := resolver.billingService.ChangeSubscription(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.BillingSubscription{
		Plan: &model.BillingPlan{
			ID:          newPlan.ID,
			Name:        newPlan.Name,
			Description: newPlan.Description,
			Product:     model.BillingProduct(newPlan.Product),
			Price:       newPlan.Price,
			IsPublic:    newPlan.IsPublic,
			Storage:     newPlan.Storage,
		},
		UpdatedAt:   customer.SubscriptionUpdatedAt,
		UsedStorage: customer.UsedStorage,
	}
	return
}
