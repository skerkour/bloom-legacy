package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

func DeclineInvitation(input model.DeclineGroupInvitationInput) (bool, error) {
	client := api.Client()

	var resp struct {
		Success bool `json:"declineGroupInvitation"`
	}
	req := graphql.NewRequest(`
	mutation($input: DeclineGroupInvitationInput!) {
		declineGroupInvitation(input: $input)
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.Success, err
}
