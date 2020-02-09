package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func RevokeSession(params RevokeSessionParams) error {
	client := api.Client()

	var resp struct {
		RevokeSession bool `json:"revokeSession"`
	}
	input := model.RevokeSessionInput{
		ID: params.ID,
	}
	req := graphql.NewRequest(`
	mutation ($input: RevokeSessionInput!) {
		revokeSession(input: $input)
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)
	if err == nil && client.SessionID != nil && params.ID == *client.SessionID {
		client.Deauthenticate()
	}

	return err
}
