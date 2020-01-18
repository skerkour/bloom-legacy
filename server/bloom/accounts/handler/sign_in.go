package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) SignIn(ctx context.Context, params *rpc.SignInParams) (*rpc.Session, error) {
	var ret rpc.Session
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("accounts.SignIn: error getting apiCtx from context")
		return &ret, twirp.InternalError(accounts.ErrorCreatePendingAccountMsg)
	}
	if apiCtx.AuthenticatedAccount != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("accounts.SignIn: generating random int", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorCreatePendingAccountMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	ret = rpc.Session{
		Id:    "MyRandomId",
		Token: "MyRandomToken",
	}
	return &ret, nil
}
