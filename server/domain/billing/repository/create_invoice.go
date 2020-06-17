package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
)

func (repo *BillingRepository) CreateInvoice(ctx context.Context, db db.Queryer, invoice billing.Invoice) (err error) {
	return
}

/*

	queryCreate := `INSERT INTO billing_invoices
	(id, created_at, updated_at, paid_at, amount, stripe_id, stripe_hosted_url, stripe_pdf_url, customer_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryCreate, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.PaidAt, ret.Amount, ret.StripeID,
		ret.StripeHostedURL, ret.StripePdfURL, ret.CustomerID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.CreateInvoice: inserting new invoice", rz.Err(err))
		return ret, NewError(ErrorCreatingInvoice)
	}

*/
