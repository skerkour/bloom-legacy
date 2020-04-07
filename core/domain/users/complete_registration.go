package users

import (
	"context"
	"crypto/rand"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/coreutil"
	"gitlab.com/bloom42/lily/graphql"
)

func CompleteRegistration(params CompleteRegistrationParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn

	authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	if authKey == nil {
		return ret, errors.New("Internal error. Please try again")
	}

	passKey := derivePassKey([]byte(params.Username), []byte(params.Password))
	if authKey == nil {
		return ret, errors.New("Internal error. Please try again")
	}

	publicKey, privateKey, err := generateKeyPair(rand.Reader)
	if err != nil {
		return ret, err
	}

	// TODO: persisr keypair

	encryptedPrivateKey, err := encryptWithPassKey(passKey, privateKey[:])
	if err != nil {
		return ret, err
	}

	input := model.CompleteRegistrationInput{
		ID:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(coreutil.GetDeviceOS()),
			Type: model.SessionDeviceType(coreutil.GetDeviceType()),
		},
		PublicKey:           publicKey[:],
		EncryptedPrivateKey: encryptedPrivateKey,
	}
	var resp struct {
		CompleteRegistration *model.SignedIn `json:"completeRegistration"`
	}
	req := graphql.NewRequest(`
        mutation ($input: CompleteRegistrationInput!) {
			completeRegistration (input: $input) {
				session {
					id
					token
				}
				me {
					id
					username
					displayName
					isAdmin
					avatarUrl
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)
	if resp.CompleteRegistration != nil {
		if resp.CompleteRegistration.Session != nil && resp.CompleteRegistration.Session.Token != nil {
			client.Authenticate(resp.CompleteRegistration.Session.ID, *resp.CompleteRegistration.Session.Token)
			err = PersistSession(resp.CompleteRegistration)
			if err != nil {
				return ret, err
			}
			ret = *resp.CompleteRegistration
			ret.Session.Token = nil
		}
	}

	return ret, err
}
