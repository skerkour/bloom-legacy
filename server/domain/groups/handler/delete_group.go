package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) DeleteGroup(ctx context.Context, params *rpc.DeleteGroupParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
