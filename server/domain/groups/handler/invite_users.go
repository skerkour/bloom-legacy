package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) InviteUsers(ctx context.Context, params *rpc.InviteUsersParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
