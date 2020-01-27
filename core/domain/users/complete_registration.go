package users

import (
	"context"
	"errors"
	"net/http"

	"gitlab.com/bloom42/bloom/common/rpc/users"
)

func CompleteRegistration(params CompleteRegistrationParams) (Session, error) {
	client := users.NewUsersProtobufClient("http://localhost:8000", &http.Client{})

	authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	if authKey == nil {
		return Session{}, errors.New("Error deriving auth key")
	}
	rpcParams := users.CompleteRegistrationParams{
		Id:       params.ID,
		Username: params.Username,
		AuthKey:  authKey,
	}

	session, err := client.CompleteRegistration(context.Background(), &rpcParams)
	if err != nil {
		return Session{}, err
	}
	return Session{
		ID:    session.Id,
		Token: session.Token,
	}, nil
}
