package handler

import (
	"context"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
)

func (handler Handler) QuitGroup(ctx context.Context, params *rpc.QuitGroupParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}

	return ret, nil
}
