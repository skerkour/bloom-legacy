package groups

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func InviteUser(params messages.InviteUserInGroupParams) (*model.Group, error) {
	client := api.Client()

	// clean input
	params.Username = strings.ToLower(strings.TrimSpace(params.Username))

	// fetch user's public key

	// generate ephemeral keypair

	// encrypt group's masterKey

	// sign invitation

	input := model.InviteUserInGroupInput{
		Username: params.Username,
		GroupID:  params.GroupID,
	}
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
