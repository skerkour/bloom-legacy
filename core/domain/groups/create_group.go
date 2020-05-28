package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/keys"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
)

func CreateGroup(input model.CreateGroupInput) (*model.Group, error) {
	client := api.Client()
	var err error
	ctx := context.Background()

	userMasterKey, err := keys.FindUserMasterKey(ctx, nil)
	defer crypto.Zeroize(userMasterKey)
	if err != nil {
		return nil, err
	}

	// generate and save a random master key
	groupMasterKey, err := crypto.NewAEADKey()
	defer crypto.Zeroize(groupMasterKey)
	if err != nil {
		return nil, err
	}

	encryptedGroupMasterKey, nonce, err := crypto.AEADEncrypt(userMasterKey, groupMasterKey, nil)
	defer crypto.Zeroize(encryptedGroupMasterKey)
	defer crypto.Zeroize(nonce)
	if err != nil {
		return nil, err
	}

	input.EncryptedMasterKey = encryptedGroupMasterKey
	input.MasterKeyNonce = nonce

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

	err = client.Do(context.Background(), req, &resp)
	if err == nil {
		group := resp.CreateGroup
		queryInsert := `INSERT INTO groups (id, created_at, name, description, avatar_url, master_key)
		VALUES (?, ?, ?, ?, ?, ?)`
		_, err = db.DB.Exec(queryInsert, group.ID, group.CreatedAt, group.Name, group.Description,
			group.AvatarURL, groupMasterKey)
	}

	return &resp.CreateGroup, err
}
