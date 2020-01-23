package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) RemoveMembers(ctx context.Context, params *rpc.RemoveMembersParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
