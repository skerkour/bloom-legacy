package billing

import (
	"time"

	"gitlab.com/bloom42/gobox/uuid"
)

type Plan struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	Name        string `db:"name"`
	StripeID    string `db:"stripe_id"`
	Price       int64  `db:"price"`
	Description string `db:"description"`
	Product     string `db:"product"`
	Storage     int64  `db:"storage"`
}

type Invoice struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	Amount          int64      `db:"amount"`
	StripeID        string     `db:"stripe_id"`
	StripeHostedURL string     `db:"stripe_hosted_url"`
	StripePdfURL    string     `db:"stripe_pdf_url"`
	PaidAt          *time.Time `db:"paid_at"`

	CustomerID uuid.UUID `db:"customer_id"`
}

type Customer struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	Email                 string    `db:"email"`
	StripeCustomerID      *string   `db:"stripe_customer_id"`
	StripeSubscriptionID  *string   `db:"stripe_subscription_id"`
	UsedStorage           int64     `db:"used_storage"`
	SubscriptionUpdatedAt time.Time `db:"subscription_updated_at"`

	PlanID  uuid.UUID  `db:"plan_id"`
	UserID  *uuid.UUID `db:"user_id"`
	GroupID *uuid.UUID `db:"group_id"`
}

type PaymentMethod struct {
	ID        uuid.UUID `db:"id"`
	CreatedAt time.Time `db:"created_at"`
	UpdatedAt time.Time `db:"updated_at"`

	IsDefault           bool   `db:"is_default"`
	StripeID            string `db:"stripe_id"`
	CardLast4           string `json:"card_last_4" db:"card_last_4"`
	CardExpirationMonth int64  `db:"card_expiration_month"`
	CardExpirationYear  int64  `db:"card_expiration_year"`

	CustomerID uuid.UUID `db:"customer_id"`
}
