package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindPaymentMethodById(ctx context.Context, tx *sqlx.Tx, id uuid.UUID) (*PaymentMethod, error) {
	var ret *PaymentMethod
	var paymentMethod PaymentMethod
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_payment_methods WHERE id = $1"
	err = tx.Get(&paymentMethod, queryFind, id)
	if err != nil {
		logger.Error("billing.FindPaymentMethodById: finding payment method", rz.Err(err),
			rz.String("payment_method.id", id.String()))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	ret = &paymentMethod
	return ret, nil
}
