package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchGroupMembers(params FetchGroupMembersParams) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"group"`
	}
	req := graphql.NewRequest(`
	query($input: ID!) {
		group(id: $input) {
			members {
				edges {
					role
					joinedAt
					node {
						avatarUrl
						username
						displayName
					}
				}
				totalCount
			}
		}
	}
	`)
	req.Var("input", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
