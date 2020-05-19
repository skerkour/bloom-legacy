package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/graphql"
)

func AcceptInvitation(input model.AcceptGroupInvitationInput) (*model.Group, error) {
	client := api.Client()

	var resp struct {
		Group *model.Group `json:"acceptGroupInvitation"`
	}
	req := graphql.NewRequest(`
	mutation($input: AcceptGroupInvitationInput!) {
		acceptGroupInvitation(input: $input) {
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
		_, err = db.DB.Exec(`INSERT INTO groups (id, created_at, name, description, avatar_url)
			VALUES (?, ?, ?, ?, ?)`, group.ID, group.CreatedAt, group.Name, group.Description, group.AvatarURL)
	}

	return resp.Group, err
}
