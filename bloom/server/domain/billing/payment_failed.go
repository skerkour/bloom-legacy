package billing

import (
	"context"

	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/libs/rz-go"
)

func PaymentFailed(ctx context.Context, stripeInvoice *stripe.Invoice) error {
	var err error
	logger := rz.FromCtx(ctx)
	var customer *Customer

	if stripeInvoice == nil || stripeInvoice.ID == "" {
		logger.Error("", rz.Err(NewError(ErrorInvoiceIsNull)))
		return NewError(ErrorUpdatingInvoice)
	}

	customer, err = FindCustomerByStripeCustomerIdNoTx(ctx, stripeInvoice.Customer.ID)
	if err != nil {
		return NewError(ErrorUpdatingInvoice)
	}

	// send Email
	go func() {
		err = SendPaymentFailedEmail(*&customer.Email, stripeInvoice.AmountDue)
		if err != nil {
			logger.Error("Error sending payment failed email", rz.Err(err))
		}
	}()

	return nil
}
