package users

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/keys"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
)

func CompleteRegistration(params CompleteRegistrationParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn
	ctx := context.Background()

	passwordKey, err := derivePasswordKeyFromPassword([]byte(params.Password), []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	// clean password from memory as we can...
	params.Password = ""
	defer crypto.Zeroize(passwordKey)

	authKey, err := deriveAuthKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	defer crypto.Zeroize(authKey)

	wrapKey, err := deriveWrapKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	// clean passwordKey from memory
	defer crypto.Zeroize(wrapKey)

	// generate and save a random master key
	masterKey, err := crypto.NewAEADKey()
	if err != nil {
		return ret, err
	}
	defer crypto.Zeroize(masterKey)

	err = keys.SaveUserMasterKey(ctx, nil, masterKey)
	if err != nil {
		return ret, err
	}
	encryptedMasterKey, masterKeyNonce, err := Encrypt(wrapKey, []byte(masterKey))
	if err != nil {
		return ret, err
	}

	// generate and save keyPair
	publicKey, privateKey, err := crypto.GenerateKeyPair(crypto.RandReader())
	if err != nil {
		return ret, err
	}
	defer crypto.Zeroize(privateKey)

	err = keys.SaveUserPublicKey(ctx, nil, publicKey)
	if err != nil {
		return ret, err
	}
	err = keys.SaveUserPrivateKey(ctx, nil, privateKey)
	if err != nil {
		return ret, err
	}

	// encrypt privateKey
	encryptedPrivateKey, privateKeyNonce, err := Encrypt(masterKey, []byte(privateKey))
	if err != nil {
		return ret, err
	}

	// prepare API request
	input := model.CompleteRegistrationInput{
		ID:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(kernel.GetDeviceOS()),
			Type: model.SessionDeviceType(kernel.GetDeviceType()),
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
					state
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(ctx, req, &resp)
	if err != nil {
		return ret, err
	}

	if resp.CompleteRegistration != nil {
		if resp.CompleteRegistration.Session != nil && resp.CompleteRegistration.Session.Token != nil {
			client.Authenticate(resp.CompleteRegistration.Session.ID, *resp.CompleteRegistration.Session.Token)
			err = SaveSignedIn(ctx, nil, resp.CompleteRegistration)
			if err != nil {
				return ret, err
			}
			ret = *resp.CompleteRegistration
			ret.Session.Token = nil
			kernel.Me = resp.CompleteRegistration.Me
		}
	}

	return ret, err
}
