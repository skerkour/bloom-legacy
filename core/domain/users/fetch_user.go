package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
)

func FetchUser(params FetchUserParams) (*model.User, error) {
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
		}
	}
	`)
	req.Var("username", params.Username)

	err := client.Do(context.Background(), req, &resp)

	return resp.User, err
}

func FetchUsers() (*model.UserConnection, error) {
	client := api.Client()

	var resp struct {
		Users *model.UserConnection `json:"users"`
	}
	req := graphql.NewRequest(`
	query {
		users {
			edges {
				node {
					id
					createdAt
					avatarUrl
					displayName
					bio
					email
					firstName
					lastName
					username
				}
			}
			totalCount
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.Users, err
}

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
			disabledAt
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
