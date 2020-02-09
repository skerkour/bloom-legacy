package users

import (
	"context"

	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

// See https://bloom.sh/the-guide/projects/bloom/cryptography#registration for the spec
func StartRegistration(params StartRegistrationParams) (model.RegistrationStarted, error) {
	client := api.Client()
	var ret model.RegistrationStarted

	if err := validator.UserDisplayName(params.DisplayName); err != nil {
		return model.RegistrationStarted{}, err
	}

	input := model.RegisterInput{
		Email:       params.Email,
		DisplayName: params.DisplayName,
	}
	var resp struct {
		Register *model.RegistrationStarted `json:"register"`
	}
	req := graphql.NewRequest(`
        mutation ($input: RegisterInput!) {
			register (input:$input) {
				id
			}
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)
	if resp.Register != nil {
		ret = *resp.Register
	}

	return ret, err
}
