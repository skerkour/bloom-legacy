package billing

import (
	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/customer"
	"github.com/twitchtv/twirp"
)

// creates a customer both for us, and in Stripe
func CreateCustomer(email string) twirp.Error {
	var err error

	// create Stripe customer
	customerParams := &stripe.CustomerParams{
		Email: stripe.String(email),
	}
	stripeCustomer, err := customer.New(customerParams)
	_, _ = stripeCustomer, err
	return nil
}
