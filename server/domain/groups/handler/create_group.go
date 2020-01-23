package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) CreateGroup(ctx context.Context, params *rpc.CreateGroupParams) (*rpc.Group, error) {
	ret := &rpc.Group{}

	return ret, nil
}
