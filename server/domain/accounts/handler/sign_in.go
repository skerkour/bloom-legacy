package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/accounts"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) SignIn(ctx context.Context, params *rpc.SignInParams) (*rpc.NewSession, error) {
	var ret rpc.NewSession
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("accounts.SignIn: error getting apiCtx from context")
		return &ret, twirp.InternalError(accounts.ErrorSingingInMsg)
	}
	if apiCtx.AuthenticatedAccount != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("accounts.SignIn: generating random int", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorSingingInMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("accounts.SignIn: Starting transaction", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorSingingInMsg)
	}

	// fetch account
	var account accounts.Account
	err = tx.Get(&account, "SELECT * FROM accounts WHERE username = $1 FOR UPDATE", params.Username)
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.SignIn: finding account", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}

	// verify password
	if !argon2id.VerifyPassword(params.AuthKey, account.AuthKeyHash) {
		if err != nil {
			tx.Rollback()
			return &ret, twirp.NewError(twirp.PermissionDenied, "Invalid Username / Password combination")
		}
	}

	newSession, token, twerr := accounts.StartSession(ctx, tx, account.ID, apiCtx.IP, apiCtx.UserAgent)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.SignIn: committing transaction", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorSingingInMsg)
	}

	ret = rpc.NewSession{
		Id:    newSession.ID,
		Token: token,
	}
	return &ret, nil
}
