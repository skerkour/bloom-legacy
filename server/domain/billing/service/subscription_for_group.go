package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) SubscriptionForGroup(ctx context.Context, groupID uuid.UUID) (customer billing.Customer, plan billing.Plan, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	err = service.groupsService.CheckUserIsGroupAdmin(ctx, service.db, me.ID, groupID)
	if err != nil {
		return
	}

	customer, err = service.billingRepo.FindCustomerByGroupID(ctx, service.db, groupID)
	if err != nil {
		return
	}

	plan, err = service.billingRepo.FindPlanForCustomer(ctx, tx, customer.ID)
	return
}
