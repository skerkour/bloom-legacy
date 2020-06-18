package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) FindPaymentMethodsForUser(ctx context.Context, userID uuid.UUID) (ret []billing.PaymentMethod, err error) {
	ret = []billing.PaymentMethod{}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if me.ID != userID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	ret, err = service.billingRepo.FindPaymentMethodsForUser(ctx, service.db, userID)
	return
}
