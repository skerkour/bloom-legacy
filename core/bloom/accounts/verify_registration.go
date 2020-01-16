package accounts

import (
	"context"
	"gitlab.com/bloom42/bloom/core/bloom/kernel"
	"net/http"

	"gitlab.com/bloom42/bloom/core/rpc/accounts"
)

func VerifyRegistration(params VerifyRegistrationParams) (kernel.Empty, error) {
	client := accounts.NewAccountsProtobufClient("http://localhost:8000", &http.Client{})

	rpcParams := accounts.VerifyRegistrationParams{
		Code: params.Code,
		Id:   params.ID,
	}

	_, err := client.VerifyRegistration(context.Background(), &rpcParams)
	if err != nil {
		return kernel.Empty{}, err
	}

	return kernel.Empty{}, nil
}
