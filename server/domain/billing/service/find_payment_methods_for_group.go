package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindPaymentMethodsForGroup(ctx context.Context, groupID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	err = service.groupsService.CheckUserIsGroupAdmin(ctx, service.db, me.ID, groupID)
	if err != nil {
		return
	}

	ret, err = service.billingRepo.FindPaymentMethodsForGroup(ctx, service.db, groupID)
	return
}
