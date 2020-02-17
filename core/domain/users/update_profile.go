package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func UpdateProfile(input model.UserProfileInput) (*model.User, error) {
	client := api.Client()

	var resp struct {
		UpdateUserProfile *model.User `json:"updateUserProfile"`
	}
	req := graphql.NewRequest(`
	mutation ($input: UserProfileInput!) {
		updateUserProfile(input: $input) {
			id
			displayName
			bio
			firstName
			lastName
		}
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.UpdateUserProfile, err
}
