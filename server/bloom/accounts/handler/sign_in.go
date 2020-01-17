package handler

import (
	"context"

	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
)

func (s Handler) SignIn(ctx context.Context, params *rpcaccounts.SignInParams) (*rpcaccounts.Session, error) {
	return &rpcaccounts.Session{
		Id:    "MyRandomId",
		Token: "MyRandomToken",
	}, nil
}
