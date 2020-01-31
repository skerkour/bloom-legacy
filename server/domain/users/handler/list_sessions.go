package handler

import (
	"context"
	"time"

	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) ListSessions(ctx context.Context, _ *rpc.Empty) (*rpc.Sessions, error) {
	ret := &rpc.Sessions{Sessions: []*rpc.Session{}}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	sessions := []users.Session{}
	err := db.DB.Select(&sessions, "SELCT * FROM sessions WHERE user_id = $1", apiCtx.AuthenticatedUser.ID)
	if err != nil {
		logger.Error("users.ListSessions: fetching sessions", rz.Err(err))
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
