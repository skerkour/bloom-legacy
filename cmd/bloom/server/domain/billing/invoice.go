package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/rz"
)

type Invoice struct {
	ID        string    `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	Amount          int64      `db:"amount"`
	StripeID        string     `db:"stripe_id"`
	StripeHostedURL string     `db:"stripe_hosted_url"`
	StripePdfURL    string     `db:"stripe_pdf_url"`
	PaidAt          *time.Time `db:"paid_at"`

	CustomerID string `db:"customer_id"`
}

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
