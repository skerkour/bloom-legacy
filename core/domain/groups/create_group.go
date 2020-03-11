package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
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
		group := resp.CreateGroup
		_, err = db.DB.Exec(`INSERT INTO groups (id, created_at, name, description, avatar_url)
			VALUES (?, ?, ?, ?, ?)`, group.ID, group.CreatedAt, group.Name, group.Description, group.AvatarURL)
	}

	return resp.CreateGroup, err
}
