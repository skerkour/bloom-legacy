package billing

import (
	"time"
)

type Customer struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	StripeID *string `json:"stripe_id" db:"stripe_id"`

	PlanID  string  `json:"plan_id" db:"plan_id"`
	UserID  *string `json:"user_id" db:"user_id"`
	GroupID *string `json:"group_id" db:"group_id"`
}

type Plan struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Name        string  `json:"name" db:"name"`
	StripeID    string  `json:"stripe_id" db:"stripe_id"`
	Price       float64 `json:"price" db:"price"`
	Description string  `json:"description" db:"description"`
	IsActive    bool    `json:"is_active" db:"is_active"`
	Tier        string  `json:"tier" db:"tier"`
}

type PaymentMethods struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	IsDefault           bool   `json:"is_default" db:"is_default"`
	StripeID            string `json:"stripe_id" db:"stripe_id"`
	CardLast4           string `json:"card_last_4" db:"card_last_4"`
	CardExpirationMonth int64  `json:"card_expiration_month" db:"card_expiration_month"`
	CardExpirationYear  int64  `json:"card_expiration_year" db:"card_expiration_year"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	StripeID string `json:"stripe_id" db:"stripe_id"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}
