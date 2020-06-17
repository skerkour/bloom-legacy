package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (service *BillingService) ChangeSubscription(ctx context.Context, params billing.ChangeSubscriptionParams) (customer billing.Customer, plan billing.Plan, err error) {
	return
}
