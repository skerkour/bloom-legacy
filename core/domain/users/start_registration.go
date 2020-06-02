package users

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

// StartRegistration starts the registration process.
// See https://bloom.sh/the-guide/projects/bloom/cryptography#registration for the spec
func StartRegistration(params StartRegistrationParams) (model.RegistrationStarted, error) {
	client := api.Client()
	ctx := context.Background()
	var ret model.RegistrationStarted

	email := strings.ToLower(params.Email)
	email = strings.TrimSpace(email)
	displayName := strings.TrimSpace(params.DisplayName)

	input := model.StartRegistrationInput{
		Email:       email,
		DisplayName: displayName,
	}
	var resp struct {
		StartRegistration *model.RegistrationStarted `json:"startRegistration"`
	}
	req := graphql.NewRequest(`
        mutation ($input: StartRegistrationInput!) {
			startRegistration (input:$input) {
				id
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(ctx, req, &resp)
	if err != nil {
		return ret, err
	}

	if resp.StartRegistration != nil {
		ret = *resp.StartRegistration
	}

	return ret, err
}
