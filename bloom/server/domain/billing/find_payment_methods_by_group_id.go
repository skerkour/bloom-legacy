package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
)

func FindPaymentMethodsByGroupId(ctx context.Context, tx *sqlx.Tx, groupId string) ([]PaymentMethod, error) {
	ret := []PaymentMethod{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT billing_payment_methods.* FROM billing_payment_methods
		INNER JOIN billing_customers ON billing_payment_methods.customer_id = billing_customers.id
		WHERE billing_customers.group_id = $1`
	if tx == nil {
		err = db.DB.Select(&ret, query, groupId)
	} else {
		err = tx.Select(&ret, query, groupId)
	}
	if err != nil {
		logger.Error("finding payment methods", rz.Err(err),
			rz.String("group.id", groupId))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	return ret, nil
}
