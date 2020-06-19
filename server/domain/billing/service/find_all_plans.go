package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (service *BillingService) FindAllPlans(ctx context.Context) (ret []billing.Plan, err error) {
	ret = []billing.Plan{}
	ret, err = service.billingRepo.FindAllPlans(ctx, service.db)
	return
}
