package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) CreatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod billing.PaymentMethod) (err error) {
	query := `INSERT INTO billing_payment_methods
	(id, created_at, updated_at, is_default, stripe_id, card_last_4, card_expiration_month, card_expiration_year, customer_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`

	_, err = db.Exec(ctx, query, paymentMethod.ID, paymentMethod.CreatedAt, paymentMethod.UpdatedAt,
		paymentMethod.IsDefault, paymentMethod.StripeID, paymentMethod.CardLast4,
		paymentMethod.CardExpirationMonth, paymentMethod.CardExpirationYear, paymentMethod.CustomerID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.CreatePaymentMethod: inserting payment_method"
		logger.Error(errMessage, log.Err("error", err), log.UUID("payment_method.id", paymentMethod.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
