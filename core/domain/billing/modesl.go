package billing

type NewStripePaymentMethod struct {
	Type string        `json:"type"`
	Card NewStripeCard `json:"card"`
}

type NewStripeCard struct {
	Number   string `json:"number"`
	ExpMonth string `json:"exp_month"`
	ExpYear  string `json:"exp_year"`
	Cvc      string `jsjon:"cvc"`
}

type StripeCard struct {
	ID string `json:"id"`
}
