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

func SignIn(params SignInParams) (model.SignedIn, error) {
	client := api.Client()
	var ret model.SignedIn

	passwordKey, err := derivePasswordKeyFromPassword([]byte(params.Password), []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}
	// clean password from memory as we can...
	params.Password = ""

	authKey, err := deriveAuthKeyFromPasswordKey(passwordKey, []byte(params.Username))
	if err != nil {
		return ret, errors.New("Internal error. Please try again")
	}

	// clean passwordKey from memory
	crypto.Zeroize(passwordKey)

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
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)
	if resp.SignIn != nil {
		if resp.SignIn.Session != nil && resp.SignIn.Session.Token != nil {
			client.Authenticate(resp.SignIn.Session.ID, *resp.SignIn.Session.Token)
			err = PersistSession(resp.SignIn)
			if err != nil {
				return ret, err
			}
			ret = *resp.SignIn
			ret.Session.Token = nil
		}
	}
	return ret, err
}
