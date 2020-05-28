package groups

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/keys"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
)

func AcceptInvitation(invitation model.GroupInvitation) (*model.Group, error) {
	client := api.Client()
	ctx := context.Background()
	var err error

	if err = validateAcceptInvitationParams(invitation); err != nil {
		return nil, err
	}

	myPrivateKey, err := keys.FindUserPrivateKey(ctx, nil)
	defer crypto.Zeroize(myPrivateKey)
	if err != nil {
		return nil, err
	}

	myPublicKey, err := keys.FindUserPublicKey(ctx, nil)
	defer crypto.Zeroize(myPublicKey)
	if err != nil {
		return nil, err
	}

	// verify signatures
	inviterPublicKey := crypto.PublicKey(invitation.Inviter.PublicKey)

	verified, err := inviterPublicKey.Verify(*invitation.EncryptedMasterKey, *invitation.EncryptedMasterKeySignature)
	if err != nil {
		return nil, err
	}
	if !verified {
		return nil, errors.New("Group's master key signature is not valid")
	}

	verified, err = VerifyInvitationSignature(inviterPublicKey, *invitation.Signature, *invitation.Group.ID,
		kernel.Me.Username, myPublicKey, *invitation.EphemeralPublicKey)
	if !verified {
		return nil, errors.New("Group's invitation signature is not valid")
	}

	// decrypt group's key
	groupMasterKey, err := myPrivateKey.DecryptAnonymous(*invitation.EncryptedMasterKey, *invitation.EphemeralPublicKey)
	defer crypto.Zeroize(groupMasterKey)
	if err != nil {
		return nil, err
	}

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
		queryInsert := `INSERT INTO groups (id, created_at, name, description, avatar_url, master_key, state)
		VALUES (?, ?, ?, ?, ?, ?, ?)`
		_, err = db.DB.Exec(queryInsert, group.ID, group.CreatedAt, group.Name, group.Description,
			group.AvatarURL, groupMasterKey, "")
	}

	return resp.Group, err
}

func validateAcceptInvitationParams(params model.GroupInvitation) (err error) {
	if params.EncryptedMasterKey == nil {
		err = errors.New("Encrypted master key is null")
		return
	}
	if params.EphemeralPublicKey == nil {
		err = errors.New("Ephemeral public key is null")
		return
	}
	if params.Group == nil {
		err = errors.New("Group is null")
		return
	}
	if params.EncryptedMasterKeySignature == nil {
		err = errors.New("Encrypted master key signature is null")
		return
	}
	if params.Signature == nil {
		err = errors.New("Invitation signature is null")
		return
	}
	if params.Inviter == nil {
		err = errors.New("Inviter is null")
		return
	}
	if params.Inviter.PublicKey == nil {
		err = errors.New("Inviter's public key is null")
		return
	}
	return
}
