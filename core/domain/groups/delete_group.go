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

	err = client.Do(context.Background(), req, &resp)
	if err == nil {
		_, err = db.DB.Exec("DELETE FROM groups WHERE id = ?", input.ID)
		db.DB.Exec("DELETE FROM objects WHERE group_id = ?", input.ID)
	}

	return err
}
