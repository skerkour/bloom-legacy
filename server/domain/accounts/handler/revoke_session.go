package handler

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	rpcaccounts "gitlab.com/bloom42/bloom/common/rpc/accounts"
)

func (s Handler) RevokeSession(ctx context.Context, params *rpcaccounts.RevokeSessionParams) (*google_protobuf.Empty, error) {
	return nil, nil
}