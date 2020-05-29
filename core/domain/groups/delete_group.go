package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func DeleteGroup(params messages.GroupsDeleteParams) error {
	client := api.Client()
	var err error
	ctx := context.Background()

	input := model.DeleteGroupInput{
		ID: params.GroupID,
	}
	var resp struct {
		DeleteGroup bool `json:"deleteGroup"`
	}
	req := graphql.NewRequest(`
	mutation ($input: DeleteGroupInput!) {
		deleteGroup(input: $input)
	}
	`)
	req.Var("input", input)

	err = client.Do(ctx, req, &resp)
	if err == nil {
		_, err = db.DB.Exec("DELETE FROM groups WHERE id = ?", input.ID)
		DeleteGroupObjects(ctx, nil, input.ID)
	}

	return err
}
