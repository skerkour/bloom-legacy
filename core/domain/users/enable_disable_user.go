package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/gobox/graphql"
)

func DisableUser(params DisableUserParams) error {
	client := api.Client()

	var resp struct {
		DisableUser bool `json:"disableUser"`
	}
	req := graphql.NewRequest(`
	mutation ($id: ID!) {
		disableUser(id: $id)
	}
	`)
	req.Var("id", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return err
}

func EnableUser(params EnableUserParams) error {
	client := api.Client()

	var resp struct {
		EnableUser bool `json:"enableUser"`
	}
	req := graphql.NewRequest(`
	mutation ($id: ID!) {
		enableUser(id: $id)
	}
	`)
	req.Var("id", params.ID)

	err := client.Do(context.Background(), req, &resp)

	return err
}
