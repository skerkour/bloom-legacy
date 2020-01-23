package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) UpdateGroup(ctx context.Context, _ *rpc.Empty) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
