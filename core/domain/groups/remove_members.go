package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

func RemoveMembers(input model.RemoveGroupMembersInput) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"removeGroupMembers"`
	}
	req := graphql.NewRequest(`
	mutation($input: RemoveGroupMembersInput!) {
		removeGroupMembers(input: $input) {
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
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
