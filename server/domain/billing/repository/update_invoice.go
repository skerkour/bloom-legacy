package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) UpdateInvoice(ctx context.Context, db db.Queryer, invoice billing.Invoice) (err error) {
	query := `UPDATE billing_invoices SET
	updated_at = $1, stripe_id = $2, stripe_hosted_url = $3, stripe_pdf_url = $4, paid_at = $5
	WHERE id = $6`

	_, err = db.Exec(ctx, query, invoice.UpdatedAt, invoice.StripeID, invoice.StripeHostedURL, invoice.StripePdfURL,
		invoice.PaidAt, invoice.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "billing.UpdateInvoice: updating invoice"
		logger.Error(errMessage, log.Err("error", err), log.UUID("invoice.id", invoice.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
