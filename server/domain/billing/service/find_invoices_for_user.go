package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindInvoicesForUser(ctx context.Context, userID uuid.UUID) (ret []billing.Invoice, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if me.ID != userID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret = service.billingRepo.FindInvoicesForUser(ctx, service.db, userID)
	return
}
