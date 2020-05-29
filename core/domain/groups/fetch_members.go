package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func FetchMembers(params messages.GroupsFetchMembersParams) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"group"`
	}
	req := graphql.NewRequest(`
	query($groupID: ID!) {
		group(id: $groupID) {
			id
			name
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
			invitations {
				nodes {
					inviter {
						username
						displayName
					}
					invitee {
						username
						displayName
					}
				}
				totalCount
			}
		}
	}
	`)
	req.Var("groupID", params.GroupID)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
