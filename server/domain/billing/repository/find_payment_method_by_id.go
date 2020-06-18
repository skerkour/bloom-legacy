package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *BillingRepository) FindPaymentMethodByID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (ret billing.PaymentMethod, err error) {
	query := "SELECT * FROM billing_payment_methods WHERE id = $1"

	err = db.Get(ctx, &ret, query, paymentMethodID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Payment method not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindPaymentMethodByID: finding payment_method"
			logger.Error(errMessage, log.Err("error", err), log.UUID("payment_method.id", paymentMethodID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
