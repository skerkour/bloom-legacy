package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) SignIn(ctx context.Context, params *rpc.SignInParams) (*rpc.NewSession, error) {
	var ret rpc.NewSession
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.SignIn: error getting apiCtx from context")
		return &ret, twirp.InternalError(users.ErrorSingingInMsg)
	}
	if apiCtx.AuthenticatedUser != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.SignIn: generating random int", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorSingingInMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SignIn: Starting transaction", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorSingingInMsg)
	}

	// fetch user
	var user users.User
	err = tx.Get(&user, "SELECT * FROM users WHERE username = $1 FOR UPDATE", params.Username)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignIn: finding user", rz.Err(err))
		return &ret, twirp.NewError(twirp.PermissionDenied, "Invalid Username / Password combination")
	}

	// verify password
	if !argon2id.VerifyPassword(params.AuthKey, user.AuthKeyHash) {
		tx.Rollback()
		return &ret, twirp.NewError(twirp.PermissionDenied, "Invalid Username / Password combination")
	}

	newSession, token, twerr := users.StartSession(ctx, tx, user.ID, apiCtx.IP, apiCtx.UserAgent)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignIn: committing transaction", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorSingingInMsg)
	}

	ret = rpc.NewSession{
		Id:    newSession.ID,
		Token: token,
	}
	return &ret, nil
}
