package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
)

func InviteUsers(input model.InviteUsersInGroupInput) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"inviteUsersInGroup"`
	}
	req := graphql.NewRequest(`
	mutation($input: InviteUsersInGroupInput!) {
		inviteUsersInGroup(input: $input) {
			invitations {
				edges {
					node {
						inviter {
							username
							displayName
						}
						invitee {
							username
							displayName
						}
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
