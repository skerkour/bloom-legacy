package accounts

import (
	"context"

	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
)

func (s Handler) CompleteRegistration(ctx context.Context, params *rpcaccounts.CompleteRegistrationParams) (*rpcaccounts.Session, error) {
	return nil, nil
}
