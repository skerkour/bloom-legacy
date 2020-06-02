package billing

import (
	"gitlab.com/bloom42/bloom/core/messages"
)

type NewStripePaymentMethod struct {
	Type string                 `json:"type"`
	Card messages.NewStripeCard `json:"card"`
}

type StripeCard struct {
	ID string `json:"id"`
}

// see https://stripe.com/docs/api/errors?lang=curl
type StripeResError struct {
	Error StripeError `json:"error"`
}

type StripeError struct {
	Message string `json:"message"`
}
