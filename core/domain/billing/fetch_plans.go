package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchPlans() (*[]model.BillingPlan, error) {
	client := api.Client()

	var resp struct {
		BillingPlans *[]model.BillingPlan `json:"billingPlans"`
	}
	req := graphql.NewRequest(`
	query {
		billingPlans {
			id
			tier
			price
			name
			description
			storage
			isActive
			stripeId
			subscribers
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.BillingPlans, err
}
