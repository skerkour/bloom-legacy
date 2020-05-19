package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

func CancelInvitation(input model.CancelGroupInvitationInput) (bool, error) {
	client := api.Client()

	var resp struct {
		Success bool `json:"cancelGroupInvitation"`
	}
	req := graphql.NewRequest(`
	mutation($input: CancelGroupInvitationInput!) {
		cancelGroupInvitation(input: $input)
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.Success, err
}
