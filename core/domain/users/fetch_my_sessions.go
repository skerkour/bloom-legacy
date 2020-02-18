package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchMySessions() (model.User, error) {
	client := api.Client()

	var resp struct {
		Me model.User `json:"me"`
	}
	req := graphql.NewRequest(`
	query {
		me {
			sessions {
				edges {
					node {
						id
						createdAt
						device {
							os
							type
						}
					}
				}
			}
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.Me, err
}
