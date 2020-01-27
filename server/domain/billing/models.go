package billing

import (
	"time"
)

type Customer struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	PlanID   string  `json:"plan_id" db:"plan_id"`
	StripeID *string `json:"stripe_id" db:"stripe_id"`
	UserID   *string `json:"user_id" db:"user_id"`
	GroupID  *string `json:"group_id" db:"group_id"`
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

	IsDefault  bool   `json:"is_default" db:"is_default"`
	CustomerID string `json:"customer_id" db:"customer_id"`
}

type Invoice struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	StripeID   string `json:"stripe_id" db:"stripe_id"`
	CustomerID string `json:"customer_id" db:"customer_id"`
}
