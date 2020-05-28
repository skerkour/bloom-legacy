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

func AcceptInvitation(invitation model.GroupInvitation) (*model.Group, error) {
	client := api.Client()
	ctx := context.Background()

	// decrypt group's key
	myPrivateKey, err := keys.FindUserPrivateKey(ctx, nil)
	if err != nil {
		return nil, err
	}

	groupMasterKey, err := myPrivateKey.Decrypt(*invitation.EncryptedMasterKey, *invitation.EphemeralPublicKey)
	if err != nil {
		return nil, err
	}
	defer crypto.Zeroize(groupMasterKey)

	// encrypt group's key
	myMasterKey, err := keys.FindUserMasterKey(ctx, nil)
	defer crypto.Zeroize(myMasterKey)
	if err != nil {
		return nil, err
	}

	encryptedGroupMasterKey, nonce, err := crypto.AEADEncrypt(myMasterKey, groupMasterKey, nil)
	defer crypto.Zeroize(encryptedGroupMasterKey)
	defer crypto.Zeroize(nonce)
	if err != nil {
		return nil, err
	}

	input := model.AcceptGroupInvitationInput{
		InvitationID:       invitation.ID,
		EncryptedMasterKey: encryptedGroupMasterKey,
		MasterKeyNonce:     nonce,
	}
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

	err = client.Do(ctx, req, &resp)
	if err == nil {
		group := resp.Group
		queryInsert := `INSERT INTO groups (id, created_at, name, description, avatar_url, master_key)
		VALUES (?, ?, ?, ?, ?, ?)`
		_, err = db.DB.Exec(queryInsert, group.ID, group.CreatedAt, group.Name, group.Description,
			group.AvatarURL, groupMasterKey)
	}

	return resp.Group, err
}
