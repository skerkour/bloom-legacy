package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchMyProfile() (model.User, error) {
	client := api.Client()

	var resp struct {
		Me model.User `json:"me"`
	}
	req := graphql.NewRequest(`
	query {
		me {
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

	err := client.Do(context.Background(), req, &resp)

	return resp.Me, err
}
