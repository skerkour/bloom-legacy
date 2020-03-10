package billing

type NewStripePaymentMethod struct {
	Type string        `json:"type"`
	Card NewStripeCard `json:"card"`
}

type NewStripeCard struct {
	Number   string `json:"number"`
	ExpMonth string `json:"expMonth"`
	ExpYear  string `json:"expYear"`
	Cvc      string `jsjon:"cvc"`
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
