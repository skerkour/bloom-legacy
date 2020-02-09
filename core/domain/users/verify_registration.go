package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func VerifyRegistration(params VerifyRegistrationParams) (bool, error) {
	client := api.Client()
	ret := false

	input := model.VerifyRegistrationInput{
		ID:   params.ID,
		Code: params.Code,
	}
	var resp struct {
		VerifyRegistration *bool `json:"verifyRegistration"`
	}
	req := graphql.NewRequest(`
        mutation ($input: VerifyRegistrationInput!) {
			verifyRegistration(input: $input)
		}
	`)
	req.Var("input", input)

	err := client.Do(context.Background(), req, &resp)
	if resp.VerifyRegistration != nil {
		ret = *resp.VerifyRegistration
	}

	return ret, err
}
