package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindInvoicesForGroup(ctx context.Context, groupID uuid.UUID) (ret []billing.Invoice, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	err = service.groupsService.CheckUserIsGroupAdmin(ctx, service.db, me.ID, groupID)
	if err != nil {
		return
	}

	ret, err = service.billingRepo.FindInvoicesForGroup(ctx, service.db, groupID)
	return
}
