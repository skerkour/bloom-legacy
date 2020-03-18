package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchUserDetails(params FetchUserParams) (*model.User, error) {
	client := api.Client()

	var resp struct {
		User *model.User `json:"user"`
	}
	req := graphql.NewRequest(`
	query($username: String) {
		user(username: $username) {
			id
			createdAt
			avatarUrl
			displayName
			bio
			email
			firstName
			lastName
			username
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
						paid
					}
				}
			}
			groups {
				edges {
					node {
						id
						createdAt
						avatarUrl
						name
						description
						members {
							totalCount
						}
					}
				}
			}
		}
	}
	`)
	req.Var("username", params.Username)

	err := client.Do(context.Background(), req, &resp)

	return resp.User, err
}
