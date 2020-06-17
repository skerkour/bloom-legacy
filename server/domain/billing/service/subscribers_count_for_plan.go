package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) SubscribersCountForPlan(ctx context.Context, planID uuid.UUID) (ret int64, err error) {
	return
}
