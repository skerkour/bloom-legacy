package service

import (
	"context"
	"time"

	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) CreateOrUpdateInvoice(ctx context.Context, stripeInvoice stripe.Invoice) (err error) {
	logger := log.FromCtx(ctx)
	now := time.Now().UTC()

	if stripeInvoice.ID == "" {
		logger.Error("billing.CreateOrUpdateInvoice: stripeInvoice is null")
		err = errors.Internal("", nil)
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.CreateOrUpdateInvoice: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	invoice, err := service.billingRepo.FindInvoiceByStripeInvoiceID(ctx, tx, stripeInvoice.ID)
	if err != nil {
		if _, ok := err.(*errors.NotFoundError); !ok {
			return err
		}
	}

	// invoice not found
	if err != nil {
		customer, err := service.billingRepo.FindCustomerByStripeCustomerID(ctx, tx, stripeInvoice.Customer.ID)
		if err != nil {
			tx.Rollback()
			return err
		}

		// create invoice
		var paidAt *time.Time
		if stripeInvoice.StatusTransitions.PaidAt != 0 {
			stripePaidAt := time.Unix(stripeInvoice.StatusTransitions.PaidAt, 0)
			paidAt = &stripePaidAt
		}
		invoice = billing.Invoice{
			ID:              uuid.New(),
			CreatedAt:       now,
			UpdatedAt:       now,
			PaidAt:          paidAt,
			Amount:          stripeInvoice.AmountDue,
			StripeID:        stripeInvoice.ID,
			StripeHostedURL: stripeInvoice.HostedInvoiceURL,
			StripePdfURL:    stripeInvoice.InvoicePDF,
			CustomerID:      customer.ID,
		}
		err = service.billingRepo.CreateInvoice(ctx, tx, invoice)
		if err != nil {
			tx.Rollback()
			return err
		}

	} else {
		if stripeInvoice.StatusTransitions.PaidAt != 0 {
			stripePaidAt := time.Unix(stripeInvoice.StatusTransitions.PaidAt, 0)
			invoice.PaidAt = &stripePaidAt
		}
		invoice.UpdatedAt = now
		invoice.StripePdfURL = stripeInvoice.InvoicePDF
		err = service.billingRepo.UpdateInvoice(ctx, tx, invoice)
		if err != nil {
			tx.Rollback()
			return
		}
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.CreateOrUpdateInvoice: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
