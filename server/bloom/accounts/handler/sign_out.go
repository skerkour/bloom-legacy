package handler

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/api/apictx"
)

func (s Handler) SignOut(ctx context.Context, _ *google_protobuf.Empty) (*google_protobuf.Empty, error) {
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return nil, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedAccount == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return nil, twerr
	}
	return nil, twirp.NotFoundError("lol not found")
}
