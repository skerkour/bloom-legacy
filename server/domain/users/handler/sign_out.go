package handler

import (
	"context"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) SignOut(ctx context.Context, _ *rpc.Empty) (*rpc.Empty, error) {
	ret := &rpc.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SignOut: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorDeleteSessionMsg)
	}

	twerr := users.DeleteSession(ctx, tx, apiCtx.Session.ID, apiCtx.AuthenticatedUser.ID)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignOut: committing transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorDeleteSessionMsg)
	}

	return ret, nil
}
