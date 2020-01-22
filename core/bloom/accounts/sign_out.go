package accounts

import (
	"context"
	"net/http"

	"gitlab.com/bloom42/bloom/common/rpc/accounts"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
)

func SignOut() error {
	client := accounts.NewAccountsProtobufClient("http://localhost:8080", &http.Client{})

	_, err := client.SignOut(context.Background(), &rpc.Empty{})
	return err
}
