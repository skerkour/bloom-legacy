package billing

import "gitlab.com/bloom42/bloom/server/errors"

var (
	ErrStripeIDIsNotValid            = errors.InvalidArgument("\"stripe_id\" is not valid.")
	ErrPaymentMethodIsAlreadyDefault = errors.InvalidArgument("Payment method is already the default. Please change and try again.")
	ErrPaymentMethodNotFound         = errors.NotFound("Payment method not found.")
)
