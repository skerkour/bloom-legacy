package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) ListAllGroups(ctx context.Context, _ *rpc.Empty) (*rpc.GroupList, error) {
	ret := &rpc.GroupList{}

	return ret, nil
}
