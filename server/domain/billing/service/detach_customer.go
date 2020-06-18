package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) DetachCustomer(ctx context.Context, db db.Queryer, user users.User, groupID *uuid.UUID) (err error) {
	var customer billing.Customer

	if groupID != nil {
		customer, err = service.billingRepo.FindCustomerByGroupID(ctx, db, *groupID)
		if err != nil {
			return
		}
	} else {
		customer, err = service.billingRepo.FindCustomerByUserID(ctx, db, user.ID)
		if err != nil {
			return
		}
	}

	customer.UserID = nil
	customer.GroupID = nil
	customer.UpdatedAt = time.Now().UTC()
	err = service.billingRepo.UpdateCustomer(ctx, db, customer)
	return
}
