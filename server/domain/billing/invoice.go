package billing

import (
	"time"
)

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Amount          int64  `json:"amount" db:"amount"`
	StripeID        string `json:"stripe_id" db:"stripe_id"`
	StripeHostedURL string `json:"stripe_hosted_url" db:"stripe_hosted_url"`
	StripePdfURL    string `json:"stripe_pdf_url" db:"stripe_pdf_url"`
	Paid            bool   `json:"paid" db:"paid"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}
