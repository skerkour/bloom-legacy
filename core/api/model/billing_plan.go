package model

type BillingPlan struct {
	ID          string          `json:"id"`
	Price       Int64           `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	IsPublic    bool            `json:"isPublic"`
	Product     BillingProduct  `json:"product"`
	Storage     Int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
	Subscribers *UserConnection `json:"subscribers"`
}
