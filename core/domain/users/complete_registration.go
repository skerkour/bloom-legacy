package users

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/coreutil"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/graphql"
)

func CompleteRegistration(params CompleteRegistrationParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn
	ctx := context.Background()

	passwordKey, err := derivePasswordKeyFromPassword([]byte(params.Password), []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	params.Password = "" // clean password from memory as we can...

	authKey, err := deriveAuthKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}

	wrapKey, err := deriveWrapKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}

	crypto.Zeroize(passwordKey) // clean passwordKey from memory

	publicKey, privateKey, err := crypto.GenerateKeyPair(crypto.RandReader())
	if err != nil {
		return ret, err
	}
	err = SavePublicKey(ctx, nil, publicKey)
	if err != nil {
		return ret, err
	}
	err = SavePrivateKey(ctx, nil, privateKey)
	if err != nil {
		return ret, err
	}

	// encrypt privateKey
	encryptedPrivateKey, privateKeyNonce, err := encryptWithPassKey(wrapKey, []byte(privateKey))
	if err != nil {
		return ret, err
	}
	crypto.Zeroize(privateKey)

	// generate and save a random master key
	masterKey, err := crypto.RandBytes(crypto.KeySize256)
	if err != nil {
		return ret, err
	}
	err = SaveMasterKey(ctx, nil, masterKey)
	if err != nil {
		return ret, err
	}
	encryptedMasterKey, masterKeyNonce, err := encryptWithPassKey(wrapKey, []byte(masterKey))
	if err != nil {
		return ret, err
	}
	crypto.Zeroize(masterKey)
	crypto.Zeroize(wrapKey)

	input := model.CompleteRegistrationInput{
		ID:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(coreutil.GetDeviceOS()),
			Type: model.SessionDeviceType(coreutil.GetDeviceType()),
		},
		PublicKey:           []byte(publicKey),
		EncryptedPrivateKey: encryptedPrivateKey,
		PrivateKeyNonce:     privateKeyNonce,
		EncryptedMasterKey:  encryptedMasterKey,
		MasterKeyNonce:      masterKeyNonce,
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
		crypto.Zeroize(authKey)

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
