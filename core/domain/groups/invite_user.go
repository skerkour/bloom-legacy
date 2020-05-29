package groups

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/keys"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
)

func InviteUser(params messages.GroupsInviteUserParams) (*model.Group, error) {
	client := api.Client()
	ctx := context.Background()
	var err error
	var ret *model.Group

	// clean input
	params.Username = strings.ToLower(strings.TrimSpace(params.Username))

	// find my private key
	myPrivateKey, err := keys.FindUserPrivateKey(ctx, nil)
	if err != nil {
		return ret, err
	}
	defer crypto.Zeroize(myPrivateKey)

	// fetch user's public key
	invitee, err := users.FetchUser(users.FetchUserParams{Username: params.Username})
	if err != nil {
		return ret, err
	}

	// encrypt and sign group's masterKey
	groupMasterKey, err := keys.FindGroupMasterKey(ctx, nil, params.GroupID)
	if err != nil {
		return ret, err
	}
	defer crypto.Zeroize(groupMasterKey)

	inviteePublicKey := crypto.PublicKey(invitee.PublicKey)
	encryptedMasterKey, ephemeralPublicKey, err := inviteePublicKey.EncryptAnonymous(groupMasterKey)
	if err != nil {
		return ret, err
	}

	// sign invitation
	signature, err := SignInvitation(myPrivateKey, params.GroupID, invitee.Username, invitee.PublicKey, ephemeralPublicKey, encryptedMasterKey)
	if err != nil {
		return ret, nil
	}
	defer crypto.Zeroize(signature)

	input := model.InviteUserInGroupInput{
		Username:           params.Username,
		GroupID:            params.GroupID,
		EphemeralPublicKey: ephemeralPublicKey,
		Signature:          signature,
		EncryptedMasterKey: encryptedMasterKey,
	}
	var resp struct {
		Group *model.Group `json:"inviteUserInGroup"`
	}
	req := graphql.NewRequest(`
	mutation($input: InviteUserInGroupInput!) {
		inviteUsersInGroup(input: $input) {
			invitations {
				nodes {
					inviter {
						username
						displayName
					}
					invitee {
						username
						displayName
					}
				}
				totalCount
			}
		}
	}
	`)
	req.Var("input", input)

	err = client.Do(ctx, req, &resp)
	if err != nil {
		return ret, err
	}

	ret = resp.Group
	return ret, err
}
