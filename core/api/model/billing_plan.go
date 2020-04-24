package model

import (
	"gitlab.com/bloom42/lily/uuid"
)

type BillingPlan struct {
	ID          uuid.UUID       `json:"id"`
	Price       int64           `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	IsPublic    bool            `json:"isPublic"`
	Product     BillingProduct  `json:"product"`
	Storage     int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
	Subscribers *UserConnection `json:"subscribers"`
}
