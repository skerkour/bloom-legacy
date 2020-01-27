package handler

import (
	"context"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/billing"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) ListPlans(ctx context.Context, params *rpc.Empty) (*rpc.PlanList, error) {
	ret := &rpc.PlanList{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	_ = logger

	return ret, nil
}
