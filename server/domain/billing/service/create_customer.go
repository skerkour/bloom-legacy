package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) CreateCustomer(ctx context.Context, db db.Queryer, user users.User, groupID *uuid.UUID) (ret billing.Customer, err error) {
	return
}
