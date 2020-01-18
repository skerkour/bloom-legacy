package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/accounts"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) CompleteRegistration(ctx context.Context, params *rpc.CompleteRegistrationParams) (*rpc.Session, error) {
	var ret rpc.Session
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("accounts.CompleteRegistration: error getting apiCtx from context")
		return &ret, twirp.InternalError(accounts.ErrorCreatePendingAccountMsg)
	}
	if apiCtx.AuthenticatedAccount != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("accounts.CompleteRegistration: generating random int", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorCreatePendingAccountMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("accounts.CompleteRegistration: Starting transaction", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorCompletingRegistrationMsg)
	}

	// find pending account
	var pendingAccount accounts.PendingAccount
	err = tx.Get(&pendingAccount, "SELECT * FROM pending_accounts WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.CompleteRegistration: getting pending account", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorCompletingRegistrationMsg)
	}

	// delete pending account
	twerr := accounts.DeletePendingAccount(ctx, tx, pendingAccount.ID)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	// create account
	newAccount, twerr := accounts.CreateAccount(ctx, tx, pendingAccount, params.Username, params.AuthKey)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	// start session
	newSessiont, token, twerr := accounts.StartSession(ctx, tx, newAccount.ID, apiCtx.IP, apiCtx.UserAgent)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.VerifyRegistration: Committing transaction", rz.Err(err))
		return &ret, twirp.InternalError(accounts.ErrorCompletingRegistrationMsg)
	}

	ret = rpc.Session{
		Id:    newSessiont.ID,
		Token: token,
	}
	return &ret, nil
}
