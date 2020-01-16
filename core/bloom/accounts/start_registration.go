package accounts

import (
	"context"
	"net/http"

	"gitlab.com/bloom42/bloom/core/rpc/accounts"
)

// See https://bloom.sh/the-guide/projects/bloom/security/authentication.html#registration for the spec
func StartRegistration(params StartRegistrationParams) (RegistrationStarted, error) {
	client := accounts.NewAccountsProtobufClient("http://localhost:8000", &http.Client{})

	if err := ValidateDisplayName(params.DisplayName); err != nil {
		return RegistrationStarted{}, err
	}

	rpcParams := accounts.StartRegistrationParams{
		Email:       params.Email,
		DisplayName: params.DisplayName,
	}

	res, err := client.StartRegistration(context.Background(), &rpcParams)
	if err != nil {
		return RegistrationStarted{}, err
	}

	return RegistrationStarted{ID: res.Id}, nil
}
