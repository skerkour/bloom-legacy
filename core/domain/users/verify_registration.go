package users

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/libs/graphql-go"
)

func VerifyRegistration(params VerifyRegistrationParams) (bool, error) {
	client := api.Client()
	ret := false

	replacer := strings.NewReplacer("-", "", " ", "", "	", "")
	code := replacer.Replace(params.Code)
	code = strings.ToLower(code)

	input := model.VerifyRegistrationInput{
		ID:   params.ID,
		Code: code,
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
