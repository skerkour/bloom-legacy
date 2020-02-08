package users

import (
	"context"
	"errors"

	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/coreutil"
	"gitlab.com/bloom42/libs/graphql-go"
)

func CompleteRegistration(params CompleteRegistrationParams) (model.SignedIn, error) {
	client := graphql.NewClient(consts.API_BASE_URL + "/graphql")
	var ret model.SignedIn

	authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	if authKey == nil {
		return ret, errors.New("Error deriving auth key")
	}

	input := model.CompleteRegistrationInput{
		ID:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
		Device: &model.SessionDeviceInput{
			Os:   model.SessionDeviceOs(coreutil.GetDeviceOS()),
			Type: model.SessionDeviceType(coreutil.GetDeviceType()),
		},
	}

	req := graphql.NewRequest(`
        mutation ($input: CompleteRegistrationInput!) {
			completeRegistration (input: $input) {
				session {
					id
					token
				}
				mese {
					username
					displayName
					isAdmin
				}
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &ret)

	return ret, err
}
