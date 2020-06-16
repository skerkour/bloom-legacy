package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

func FindPaymentMethodsByUserId(ctx context.Context, tx *sqlx.Tx, userId string) ([]PaymentMethod, error) {
	ret := []PaymentMethod{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT billing_payment_methods.* FROM billing_payment_methods
		INNER JOIN billing_customers ON billing_payment_methods.customer_id = billing_customers.id
		WHERE billing_customers.user_id = $1`
	if tx == nil {
		err = db.DB.Select(&ret, query, userId)
	} else {
		err = tx.Select(&ret, query, userId)
	}
	if err != nil {
		logger.Error("finding payment methods", rz.Err(err),
			rz.String("users.id", userId))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	return ret, nil
}
