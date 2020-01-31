package billing

import (
	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/customer"
)

// creates a customer only for Stripe
func CreateStripeCustomer(billingEmail, defaultStripePaymentMethod string) twirp.Error {
	var err error

	// create Stripe customer
	customerParams := &stripe.CustomerParams{
		PaymentMethod: stripe.String(defaultStripePaymentMethod),
		Email:         stripe.String(billingEmail),
		InvoiceSettings: &stripe.CustomerInvoiceSettingsParams{
			DefaultPaymentMethod: stripe.String(defaultStripePaymentMethod),
		},
	}
	stripeCustomer, err := customer.New(customerParams)
	_, _ = stripeCustomer, err
	return nil
}
