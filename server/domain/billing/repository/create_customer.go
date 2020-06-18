package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) CreateCustomer(ctx context.Context, db db.Queryer, customer billing.Customer) (err error) {
	query := `INSERT INTO billing_customers
	(id, created_at, updated_at, plan_id, stripe_customer_id, stripe_subscription_id, email, used_storage, subscription_updated_at, user_id, group_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`

	_, err = db.Exec(ctx, query, customer.ID, customer.CreatedAt, customer.UpdatedAt, customer.PlanID,
		customer.StripeCustomerID, customer.StripeSubscriptionID, customer.Email, customer.UsedStorage,
		customer.SubscriptionUpdatedAt, customer.UserID, customer.GroupID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.CreateCustomer: inserting customer"
		logger.Error(errMessage, log.Err("error", err), log.UUID("customer.id", customer.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
