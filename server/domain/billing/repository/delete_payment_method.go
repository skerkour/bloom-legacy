package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) DeletePaymentMethod(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (err error) {
	query := "DELETE FROM billing_payment_methods WHERE id = $1"

	_, err = db.Exec(ctx, query, paymentMethodID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.DeletePaymentMethod: deleting payment_method"
		logger.Error(errMessage, log.Err("error", err), log.UUID("payment_method.id", paymentMethodID))
		err = errors.Internal(errMessage, err)
	}
	return
}
