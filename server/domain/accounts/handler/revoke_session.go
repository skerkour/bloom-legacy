package handler

import (
	"context"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/accounts"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) RevokeSession(ctx context.Context, params *rpc.RevokeSessionParams) (*google_protobuf.Empty, error) {
	ret := &google_protobuf.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedAccount == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	if params.Id == apiCtx.Session.ID {
		twerr := twirp.NewError(twirp.InvalidArgument, "Deleting current session is not possible. Please sign out instead.")
		return ret, twerr
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("accounts.RevokeSession: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorDeleteSessionMsg)
	}

	twerr := accounts.DeleteSession(ctx, tx, params.Id, apiCtx.AuthenticatedAccount.ID)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.RevokeSession: committing transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorDeleteSessionMsg)
	}

	return ret, nil
}
