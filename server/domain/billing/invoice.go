package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Amount          int64  `json:"amount" db:"amount"`
	StripeID        string `json:"stripe_id" db:"stripe_id"`
	StripeHostedURL string `json:"stripe_hosted_url" db:"stripe_hosted_url"`
	StripePdfURL    string `json:"stripe_pdf_url" db:"stripe_pdf_url"`
	Paid            bool   `json:"paid" db:"paid"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}

func FindInvoicesByUserId(ctx context.Context, userId string) ([]Invoice, error) {
	ret := []Invoice{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := `SELECT billing_invoices.* FROM billing_invoices
		INNER JOIN billing_customers ON billing_invoices.customer_id = billing_customers.id
		WHERE billing_customers.user_id = $1 ORDER BY created_at DESC`
	err = db.DB.Select(&ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindInvoicesByUserId: finding invoices", rz.Err(err),
			rz.String("users_id", userId))
		return ret, NewError(ErrorInvoiceNotFound)
	}

	return ret, nil
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
