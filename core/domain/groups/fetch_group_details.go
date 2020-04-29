package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/lily/graphql"
)

func FetchGroupDetails(params messages.FetchGroupDetailsParams) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"group"`
	}
	req := graphql.NewRequest(`
	query($groupId: ID!) {
		group(id: $groupId) {
			id
			createdAt
			avatarUrl
			name
			description
			members {
				edges {
					node {
						id
						username
						avatarUrl
					}
				}
				totalCount
			}
			invitations {
				totalCount
			}
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
				edges {
					node {
						id
						createdAt
						cardLast4
						cardExpirationMonth
						cardExpirationYear
						isDefault
					}
				}
			}
			invoices {
				edges {
					node {
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
		}
	}
	`)
	req.Var("groupId", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
