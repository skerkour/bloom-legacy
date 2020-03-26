package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindInvoicesByUserId(ctx context.Context, tx *sqlx.Tx, userId string) ([]Invoice, error) {
	ret := []Invoice{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT billing_invoices.* FROM billing_invoices
		INNER JOIN billing_customers ON billing_invoices.customer_id = billing_customers.id
		WHERE billing_customers.user_id = $1 ORDER BY created_at DESC`
	if tx == nil {
		err = db.DB.Select(&ret, query, userId)
	} else {
		err = tx.Select(&ret, query, userId)
	}
	if err != nil {
		logger.Error("billing.FindInvoicesByUserId: finding invoices", rz.Err(err),
			rz.String("users_id", userId))
		return ret, NewError(ErrorInvoiceNotFound)
	}

	return ret, nil
}
