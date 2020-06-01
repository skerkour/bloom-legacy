package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func FetchGroupProfile(params messages.FetchGroupProfileParams) (messages.GroupBillingProfile, error) {
	client := api.Client()

	var resp messages.GroupBillingProfile
	req := graphql.NewRequest(`
	query($groupID: ID!) {
		group(id: $groupID) {
			id
			subscription {
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
			paymentMethods {
				nodes {
					id
					createdAt
					cardLast4
					cardExpirationMonth
					cardExpirationYear
					isDefault
				}
			}
			invoices {
				nodes {
					id
					createdAt
					amount
					stripeId
					stripeHostedUrl
					stripePdfUrl
					paidAt
				}
			}
		}
		billingPlans {
			nodes {
				id
				product
				price
				name
				description
				storage
			}
		}
		stripePublicKey
	}
	`)
	req.Var("groupID", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return resp, err
}
