package billing

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreateInvoice(ctx context.Context, stripeInvoice *stripe.Invoice) (*Invoice, error) {
	var ret *Invoice
	var err error
	logger := rz.FromCtx(ctx)
	var customer *Customer
	now := time.Now().UTC()

	if stripeInvoice == nil || stripeInvoice.ID == "" {
		logger.Error("", rz.Err(NewError(ErrorInvoiceIsNull)))
		return ret, NewError(ErrorCreatingInvoice)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.CreateInvoice: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorCreatingInvoice)
	}

	customer, err = FindCustomerByStripeCustomerId(ctx, tx, stripeInvoice.Customer.ID)
	if err != nil {
		tx.Rollback()
		return ret, NewError(ErrorCreatingInvoice)
	}

	// create invoice
	newUuid := uuid.New()
	ret = &Invoice{
		ID:              newUuid.String(),
		CreatedAt:       now,
		UpdatedAt:       now,
		Paid:            stripeInvoice.Paid,
		Amount:          stripeInvoice.AmountDue,
		StripeID:        stripeInvoice.ID,
		StripeHostedURL: stripeInvoice.HostedInvoiceURL,
		StripePdfURL:    stripeInvoice.InvoicePDF,
		CustomerID:      customer.ID,
	}
	queryCreate := `INSERT INTO billing_invoices
		(id, created_at, updated_at, paid, amount, stripe_id, stripe_hosted_url, stripe_pdf_url, customer_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryCreate, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Paid, ret.Amount, ret.StripeID,
		ret.StripeHostedURL, ret.StripePdfURL, ret.CustomerID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.CreateInvoice: inserting new invoice", rz.Err(err))
		return ret, NewError(ErrorCreatingInvoice)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.CreateInvoice: Committing transaction", rz.Err(err))
		return ret, NewError(ErrorCreatingInvoice)
	}

	return nil, nil
}
