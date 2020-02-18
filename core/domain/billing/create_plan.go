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
				product
				price
				name
				description
				storage
				stripeId
				isPublic
				subscribers {
					totalCount
				}
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.CreateBillingPlan, err
}
