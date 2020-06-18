package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) FindCustomerByStripeCustomerID(ctx context.Context, db db.Queryer, stripeCustomerID string) (ret billing.Customer, err error) {
	query := "SELECT * FROM billing_customers WHERE stripe_customer_id = $1"

	err = db.Get(ctx, &ret, query, stripeCustomerID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Customer not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindCustomerByStripeCustomerID: finding customer"
			logger.Error(errMessage, log.Err("error", err), log.String("stripe_customer.id", stripeCustomerID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
