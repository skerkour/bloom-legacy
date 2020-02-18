package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func UpdatePlan(input model.BillingPlanInput) (*model.BillingPlan, error) {
	client := api.Client()

	var resp struct {
		UpdateBillingPlan *model.BillingPlan `json:"updateBillingPlan"`
	}
	req := graphql.NewRequest(`
		mutation ($input: BillingPlanInput!) {
			updateBillingPlan(input: $input) {
				id
				product
				price
				name
				description
				storage
				isPublic
				stripeId
				subscribers {
					totalCount
				}
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.UpdateBillingPlan, err
}
