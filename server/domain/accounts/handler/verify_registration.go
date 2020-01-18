package handler

import (
	"context"
	"time"

	google_protobuf "github.com/golang/protobuf/ptypes/empty"
	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/accounts"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/accounts"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) VerifyRegistration(ctx context.Context, params *rpc.VerifyRegistrationParams) (*google_protobuf.Empty, error) {
	ret := &google_protobuf.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("accounts.VerifyRegistration: error getting apiCtx from context")
		return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}
	if apiCtx.AuthenticatedAccount != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("accounts.VerifyRegistration: generating random int", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// verify pending account
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("accounts.VerifyRegistration: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}

	var pendingAccount accounts.PendingAccount
	err = tx.Get(&pendingAccount, "SELECT * FROM pending_accounts WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.VerifyRegistration: getting pending account", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}

	twerr := accounts.VerifyPendingAccount(ctx, tx, pendingAccount, params.Code)
	if twerr != nil {
		tx.Rollback()
		tx, _ := db.DB.Beginx()
		if tx != nil {
			twerr2 := accounts.FailPendingAccountVerification(ctx, tx, pendingAccount)
			if twerr2 != nil {
				tx.Rollback()
				return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
			}
			tx.Commit()
		}
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.VerifyRegistration: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorVerifyPendingAccountMsg)
	}

	return ret, nil
}
