package users

import (
	"context"
	"net/http"

	"gitlab.com/bloom42/bloom/common/rpc/users"
	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
)

func SignOut() error {
	client := users.NewUsersProtobufClient("http://localhost:8080", &http.Client{})

	_, err := client.SignOut(context.Background(), &rpc.Empty{})
	return err
}
