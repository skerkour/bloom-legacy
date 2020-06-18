package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) SubscribersCountForPlan(ctx context.Context, planID uuid.UUID) (ret int64, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.billingRepo.GetSubscribersCountForPlan(ctx, service.db, planID)
	return
}
