package model

import (
	"gitlab.com/bloom42/gobox/uuid"
)

type BillingPlan struct {
	ID          uuid.UUID       `json:"id"`
	Price       int64           `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	Product     BillingProduct  `json:"product"`
	Storage     int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
	Subscribers *UserConnection `json:"subscribers"`
}
