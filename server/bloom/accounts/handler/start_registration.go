package handler

import (
	"context"

	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
)

func (s Handler) StartRegistration(ctx context.Context, params *rpcaccounts.StartRegistrationParams) (*rpcaccounts.RegistrationStarted, error) {
	return nil, nil
}
