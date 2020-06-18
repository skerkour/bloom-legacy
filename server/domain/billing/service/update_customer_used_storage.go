package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (service *BillingService) UpdateCustomerUsedStorage(ctx context.Context, db db.Queryer, customer billing.Customer, storage int64) (billing.Customer, error) {
	var err error

	diff := customer.UsedStorage + storage
	if diff < 0 {
		err = billing.ErrInvalidCustomerStorage
		return customer, err
	}

	customer.UpdatedAt = time.Now().UTC()
	customer.UsedStorage = diff
	err = service.billingRepo.UpdateCustomer(ctx, db, customer)
	return customer, err
}
