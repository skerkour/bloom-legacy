package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) UpdatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod billing.PaymentMethod) (err error) {
	query := `UPDATE billing_payment_methods SET
		updated_at = $1, is_default = $2
		WHERE id = $3`

	_, err = db.Exec(ctx, query, paymentMethod.UpdatedAt, paymentMethod.IsDefault, paymentMethod.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.UpdatePaymentMethod: updating payment method"
		logger.Error(errMessage, log.Err("error", err), log.UUID("payment_method.id", paymentMethod.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
