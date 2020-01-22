package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/accounts"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) ListSessions(ctx context.Context, _ *rpc.Empty) (*rpc.Sessions, error) {
	ret := &rpc.Sessions{Sessions: []*rpc.Session{}}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedAccount == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	sessions := []accounts.Session{}
	err := db.DB.Select("SELCT * FROM sessions WHERE account_id = $1", apiCtx.AuthenticatedAccount.ID)
	if err != nil {
		logger.Error("accounts.RevokeSession: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError("Internal error. Please try again.")
	}

	for _, session := range sessions {
		rpcSession := rpc.Session{
			Id:        session.ID,
			CreatedAt: session.CreatedAt.Format(time.RFC3339),
			Ip:        session.IPAddr,
		}
		ret.Sessions = append(ret.Sessions, &rpcSession)
	}

	return ret, nil
}
