package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) InviteMembers(ctx context.Context, params *rpc.InviteMembersParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
