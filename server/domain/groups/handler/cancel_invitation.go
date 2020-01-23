package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) CancelInvitation(ctx context.Context, params *rpc.CancelInvitationParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
