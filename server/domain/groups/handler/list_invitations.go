package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) ListInvitations(ctx context.Context, _ *rpc.Empty) (*rpc.InvitationList, error) {
	ret := &rpc.InvitationList{}

	return ret, nil
}
