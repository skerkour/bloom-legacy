package billing

import "gitlab.com/bloom42/bloom/server/errors"

var (
	ErrStripeIDIsNotValid = errors.InvalidArgument("\"stripe_id\" is not valid.")
)
