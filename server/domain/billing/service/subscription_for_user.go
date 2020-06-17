package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) SubscriptionForUser(ctx context.Context, userID uuid.UUID) (customer billing.Customer, plan billing.Plan, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	if me.ID != userID && !me.IsAdmin {
		err = users.ErrPermissionDenied
		return
	}

	customer, err = service.billingRepo.FindCustomerByUserID(ctx, service.db, userID)
	if err != nil {
		return
	}

	plan, err = service.billingRepo.FindPlanForCustomer(ctx, service.db, customer.ID)
	return
}
