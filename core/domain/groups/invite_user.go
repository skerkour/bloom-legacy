package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

func InviteUser(input model.InviteUserInGroupInput) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"inviteUserInGroup"`
	}
	req := graphql.NewRequest(`
	mutation($input: InviteUserInGroupInput!) {
		inviteUsersInGroup(input: $input) {
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
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.Group, err
}
