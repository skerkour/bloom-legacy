package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func FetchGroupDetails(params messages.GroupsFetchDetailsParams) (*model.Group, error) {
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
					role
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
	}
	`)
	req.Var("groupId", params.GroupID)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
