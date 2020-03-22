package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/lily/graphql"
)

func UpdateGroup(input model.GroupInput) (model.Group, error) {
	client := api.Client()

	var resp struct {
		Group model.Group `json:"updateGroup"`
	}
	req := graphql.NewRequest(`
	mutation ($input: GroupInput!) {
		updateGroup(input: $input) {
			id
			createdAt
			avatarUrl
			name
			description
			members {
				totalCount
			}
		}
	}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)
	if err == nil {
		group := resp.Group
		_, err = db.DB.Exec("UPDATE groups SET name = ?, description = ?, avatar_url = ? WHERE id = ?",
			group.Name, group.Description, group.AvatarURL, group.ID)
	}

	return resp.Group, err
}
