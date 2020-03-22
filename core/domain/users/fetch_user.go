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
