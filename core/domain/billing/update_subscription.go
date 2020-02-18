package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func UpdateSubscription(input model.UpdateBillingSubscriptionInput) (*model.BillingSubscription, error) {
	client := api.Client()

	var resp struct {
		UpdateBillingSubscription *model.BillingSubscription `json:"updateBillingSubscription"`
	}
	req := graphql.NewRequest(`
		mutation ($input: UpdateBillingSubscriptionInput!) {
			updateBillingSubscription(input: $input) {
				updatedAt
				usedStorage
				plan {
					id
					product
					price
					name
					storage
				}
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.UpdateBillingSubscription, err
}
