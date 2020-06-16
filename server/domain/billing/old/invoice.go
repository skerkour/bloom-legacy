package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
)

func FindInvoiceByStripeId(ctx context.Context, tx *sqlx.Tx, stripeId string) (*Invoice, error) {
	ret := &Invoice{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_invoices WHERE stripe_id = $1"
	err = tx.Get(ret, queryFind, stripeId)
	if err != nil {
		logger.Error("billing.FindInvoiceByStripeId: finding invoice", rz.Err(err),
			rz.String("stripe_id", stripeId))
		return ret, NewError(ErrorInvoiceNotFound)
	}

	return ret, nil
}
