package accounts

import (
	"context"
	"net/http"

	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/common/validator/accounts"
)

// See https://bloom.sh/the-guide/projects/bloom/security/authentication.html#registration for the spec
func StartRegistration(params StartRegistrationParams) (RegistrationStarted, error) {
	client := rpc.NewAccountsProtobufClient("http://localhost:8000", &http.Client{})

	if err := accounts.ValidateDisplayName(params.DisplayName); err != nil {
		return RegistrationStarted{}, err
	}

	rpcParams := rpc.StartRegistrationParams{
		Email:       params.Email,
		DisplayName: params.DisplayName,
	}

	res, err := client.StartRegistration(context.Background(), &rpcParams)
	if err != nil {
		return RegistrationStarted{}, err
	}

	return RegistrationStarted{ID: res.Id}, nil
}
