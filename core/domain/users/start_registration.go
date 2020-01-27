package users

import (
	"context"
	"net/http"

	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/common/validator"
)

// See https://bloom.sh/the-guide/projects/bloom/security/authentication.html#registration for the spec
func StartRegistration(params StartRegistrationParams) (RegistrationStarted, error) {
	client := rpc.NewUsersProtobufClient("http://localhost:8000", &http.Client{})

	if err := validator.UserDisplayName(params.DisplayName); err != nil {
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
