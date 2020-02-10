package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchMyProfile() (MyBillingProfile, error) {
	client := api.Client()

	var resp MyBillingProfile
	req := graphql.NewRequest(`
	query {
		me {
			id
			billingPlan {
				id
				tier
				price
				name
				storage
			}
			paymentMethods {
				id
				createdAt
				cardLast4
				cardExpirationMonth
				cardExpirationYear
			}
			invoices {
				id
			}
		}
		billingPlans {
			id
			tier
			price
			name
			description
			storage
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp, err
}
