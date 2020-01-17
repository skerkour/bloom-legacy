package handler

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	rpcaccounts "gitlab.com/bloom42/bloom/core/rpc/accounts"
)

func (s Handler) SendNewRegistrationCode(ctx context.Context, params *rpcaccounts.SendNewRegistrationCodeParams) (*google_protobuf.Empty, error) {
	return nil, nil
}
