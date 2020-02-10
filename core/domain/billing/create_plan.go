package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func CreatePlan(input model.BillingPlanInput) (*model.BillingPlan, error) {
	client := api.Client()

	var resp struct {
		CreateBillingPlan *model.BillingPlan `json:"createBillingPlan"`
	}
	req := graphql.NewRequest(`
		mutation ($input: BillingPlanInput!) {
			createBillingPlan(input: $input) {
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
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.CreateBillingPlan, err
}
