package model

type BillingPlan struct {
	ID          string          `json:"id"`
	Price       float64         `json:"price"`
	Name        string          `json:"name"`
	Description string          `json:"description"`
	IsActive    bool            `json:"isActive"`
	Tier        BillingPlanTier `json:"tier"`
	Storage     Int64           `json:"storage"`
	StripeID    *string         `json:"stripeId"`
	Subscribers *int64          `json:"subscribers"`
}
