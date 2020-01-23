package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) DeclineInvitation(ctx context.Context, params *rpc.DeclineInvitationParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
