package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

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
