package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchGroupDetails(params FetchGroupDetailsParams) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"groups"`
	}
	req := graphql.NewRequest(`
	query($groupId: ID!) {
		group(id: $groupId) {
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
	`)
	req.Var("groupId", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
