package accounts

import (
	"context"
	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"gitlab.com/bloom42/bloom/common/rpc/accounts"
	"net/http"
)

func SignOut() error {
	client := accounts.NewAccountsProtobufClient("http://localhost:8080", &http.Client{})

	_, err := client.SignOut(context.Background(), &google_protobuf.Empty{})
	return err
}
