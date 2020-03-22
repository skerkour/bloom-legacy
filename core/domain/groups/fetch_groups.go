package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
)

func FetchGroups() (*model.GroupConnection, error) {
	client := api.Client()

	var resp struct {
		Groups *model.GroupConnection `json:"groups"`
	}
	req := graphql.NewRequest(`
	query {
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
					invitations {
						totalCount
					}
				}
			}
			totalCount
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.Groups, err
}
