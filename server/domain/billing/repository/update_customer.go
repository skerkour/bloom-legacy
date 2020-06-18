package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) UpdateCustomer(ctx context.Context, db db.Queryer, customer billing.Customer) (err error) {
	query := `UPDATE billing_customers SET
	updated_at = $1, email = $2, stripe_customer_id = $3, stripe_subscription_id = $4, used_storage = $5,
	subscription_updated_at = $6, plan_id = $7
	WHERE id = $8`

	_, err = db.Exec(ctx, query, customer.UpdatedAt, customer.Email, customer.StripeCustomerID,
		customer.StripeSubscriptionID, customer.UsedStorage, customer.SubscriptionUpdatedAt, customer.PlanID,
		customer.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.UpdateCustomer: updating customer"
		logger.Error(errMessage, log.Err("error", err), log.UUID("customer.id", customer.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
