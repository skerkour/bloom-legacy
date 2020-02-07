package billing

import (
	"time"
)

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	StripeID string `json:"stripe_id" db:"stripe_id"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}
