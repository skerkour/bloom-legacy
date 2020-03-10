package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func CreateGroup(input model.CreateGroupInput) (model.Group, error) {
	client := api.Client()

	var resp struct {
		CreateGroup model.Group `json:"createGroup"`
	}
	req := graphql.NewRequest(`
	mutation ($input: CreateGroupInput!) {
		createGroup(input: $input) {

		}
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)

	return resp.CreateGroup, err
}
