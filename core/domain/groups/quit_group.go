package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/graphql"
)

func QuitGroup(params messages.GroupsQuitParams) error {
	client := api.Client()
	ctx := context.Background()

	input := model.QuitGroupInput{
		GroupID: params.GroupID,
	}
	var resp struct {
		Success bool `json:"quitGroup"`
	}
	req := graphql.NewRequest(`
	mutation($input: QuitGroupInput!) {
		quitGroup(input: $input)
	}
	`)
	req.Var("input", input)

	err := client.Do(ctx, req, &resp)
	if err == nil {
		_, err = db.DB.Exec("DELETE FROM groups WHERE id = ?", params.GroupID)
		// automatically deleted thanks to ON CASCADE
		// DeleteGroupObjects(ctx, nil, input.ID)
	}

	return err
}
