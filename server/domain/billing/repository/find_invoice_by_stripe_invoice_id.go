package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *BillingRepository) FindInvoiceByStripeInvoiceID(ctx context.Context, db db.Queryer, stripeInvoiceID string) (ret billing.Invoice, err error) {
	query := "SELECT * FROM billing_invoices WHERE stripe_id = $1"

	err = db.Get(ctx, &ret, query, stripeInvoiceID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Invoice not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "billing.FindInvoiceByStripeInvoiceID: finding invoice"
			logger.Error(errMessage, log.Err("error", err), log.String("stripe_invoice.id", stripeInvoiceID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
