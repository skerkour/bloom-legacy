package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) CreateCustomer(ctx context.Context, db db.Queryer, user users.User, groupID *uuid.UUID) (ret billing.Customer, err error) {
	defaultPlan, err := service.billingRepo.FindDefaultPlan(ctx, db)
	if err != nil {
		return
	}
	var userID *uuid.UUID

	if groupID == nil {
		userID = &user.ID
	}

	now := time.Now().UTC()
	ret = billing.Customer{
		ID:                    uuid.New(),
		CreatedAt:             now,
		UpdatedAt:             now,
		PlanID:                defaultPlan.ID,
		StripeCustomerID:      nil,
		StripeSubscriptionID:  nil,
		Email:                 user.Email,
		UsedStorage:           0,
		SubscriptionUpdatedAt: now,
		UserID:                userID,
		GroupID:               groupID,
	}
	err = service.billingRepo.CreateCustomer(ctx, db, ret)
	return
}
