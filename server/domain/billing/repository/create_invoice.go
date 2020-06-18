package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) CreateInvoice(ctx context.Context, db db.Queryer, invoice billing.Invoice) (err error) {
	query := `INSERT INTO billing_invoices
	(id, created_at, updated_at, paid_at, amount, stripe_id, stripe_hosted_url, stripe_pdf_url, customer_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`

	_, err = db.Exec(ctx, query, invoice.ID, invoice.CreatedAt, invoice.UpdatedAt, invoice.PaidAt,
		invoice.Amount, invoice.StripeID, invoice.StripeHostedURL, invoice.StripePdfURL, invoice.CustomerID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.CreateInvoice: inserting invoice"
		logger.Error(errMessage, log.Err("error", err), log.UUID("invoice.id", invoice.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
