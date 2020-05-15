package users

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/graphql"
)

// SignIn sign in
func SignIn(params SignInParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn
	ctx := context.Background()

	passwordKey, err := derivePasswordKeyFromPassword([]byte(params.Password), []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	// clean password from memory as we can...
	params.Password = ""
	defer crypto.Zeroize(passwordKey) // clean passwordKey from memory

	authKey, err := deriveAuthKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	defer crypto.Zeroize(authKey) // clean authKey from memory

	input := model.SignInInput{
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(kernel.GetDeviceOS()),
			Type: model.SessionDeviceType(kernel.GetDeviceType()),
		},
	}
	var resp struct {
		SignIn *model.SignedIn `json:"signIn"`
	}
	req := graphql.NewRequest(`
        mutation ($input: SignInInput!) {
			signIn(input: $input) {
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

					publicKey
					encryptedPrivateKey
					privateKeyNonce
					encryptedMasterKey
					masterKeyNonce
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(ctx, req, &resp)
	if err != nil {
		return ret, err
	}
	if resp.SignIn != nil {
		if resp.SignIn.Session != nil && resp.SignIn.Session.Token != nil {
			client.Authenticate(resp.SignIn.Session.ID, *resp.SignIn.Session.Token)
			err = SaveSignedIn(resp.SignIn)
			if err != nil {
				return ret, err
			}
			ret = *resp.SignIn
			ret.Session.Token = nil
			me := resp.SignIn.Me

			// decrypt and save keys
			wrapKey, err := deriveWrapKeyFromPasswordKey(passwordKey, []byte(params.Username))
			if err != nil {
				return ret, errors.New("Internal error. Please try again")
			}
			// clean passwordKey from memory
			defer crypto.Zeroize(wrapKey)

			// decrypt master_key
			masterKey, err := decrypt(wrapKey, *me.MasterKeyNonce, *me.EncryptedMasterKey)
			if err != nil {
				return ret, errors.New("Internal error. Please try again")
			}
			defer crypto.Zeroize(masterKey) // clean masterKey from memory

			err = SaveMasterKey(ctx, nil, masterKey)
			if err != nil {
				return ret, err
			}

			// decrypt private_key
			privateKey, err := decrypt(masterKey, *me.PrivateKeyNonce, *me.EncryptedPrivateKey)
			if err != nil {
				return ret, errors.New("Internal error. Please try again")
			}
			defer crypto.Zeroize(privateKey) // clean privateKey from memory

			// save key_pair
			err = SavePublicKey(ctx, nil, me.PublicKey)
			if err != nil {
				return ret, err
			}
			err = SavePrivateKey(ctx, nil, privateKey)
			if err != nil {
				return ret, err
			}

			kernel.Me = me
			kernel.Me.EncryptedMasterKey = nil
			kernel.Me.EncryptedPrivateKey = nil
			kernel.Me.MasterKeyNonce = nil
			kernel.Me.PrivateKeyNonce = nil
		}
	}
	return ret, err
}
