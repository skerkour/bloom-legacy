package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) GetPaymentMethodsCountForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret int64, err error) {
	query := "SELECT COUNT(*) FROM billing_payment_methods WHERE customer_id = $1"

	err = db.Get(ctx, &ret, query, customerID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.GetPaymentMethodsCountForCustomer: finding payment methods count"
		logger.Error(errMessage, log.Err("error", err), log.UUID("customer.id", customerID))
		err = errors.Internal(errMessage, err)
	}
	return
}
