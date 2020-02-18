package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchPlans() (*model.BillingPlanConnection, error) {
	client := api.Client()

	var resp struct {
		BillingPlans *model.BillingPlanConnection `json:"billingPlans"`
	}
	req := graphql.NewRequest(`
	query {
		billingPlans {
			edges {
				node {
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
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.BillingPlans, err
}
