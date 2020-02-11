package model

type BillingPlan struct {
	ID          string          `json:"id"`
	Price       int64           `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	IsPublic    bool            `json:"isPublic"`
	Tier        BillingPlanTier `json:"tier"`
	Storage     Int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
	Subscribers *int64          `json:"subscribers"`
}
