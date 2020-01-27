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

	Name                     string `json:"name" db:"name"`
	Storage                  int64  `json:"storage" db:"storage"`
	ParallelBitflowDownloads int64  `json:"parallel_bitflow_downloads" db:"parallel_bitflow_downloads"`
	StripeID                 string `json:"stripe_id" db:"stripe_id"`
}

type PaymentMethods struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	IsDefault       bool   `json:"is_default" db:"is_default"`
	StripeID        string `json:"stripe_id" db:"stripe_id"`
	CardLast4       string `json:"card_last_4" db:"card_last_4"`
	ExpirationMonth int64  `json:"expiration_month" db:"expiration_month"`
	ExpirationYear  int64  `json:"expiration_year" db:"expiration_year"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	StripeID string `json:"stripe_id" db:"stripe_id"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}
