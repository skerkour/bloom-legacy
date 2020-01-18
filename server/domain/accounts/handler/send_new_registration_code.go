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

func (s Handler) SendNewRegistrationCode(ctx context.Context, params *rpc.SendNewRegistrationCodeParams) (*google_protobuf.Empty, error) {
	ret := &google_protobuf.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("accounts.SendNewRegistrationCode: error getting apiCtx from context")
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}
	if apiCtx.AuthenticatedAccount != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("accounts.SendNewRegistrationCode: generating random int", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("accounts.SendNewRegistrationCode: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}

	var pendingAccount accounts.PendingAccount
	err = tx.Get(&pendingAccount, "SELECT * FROM pending_accounts WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.SendNewRegistrationCode: getting pending account", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}

	now := time.Now().UTC()
	since := now.Sub(pendingAccount.UpdatedAt)
	if since <= 30*time.Second {
		return ret, twirp.NewError(twirp.PermissionDenied, "Please wait at least 30 seconds before requesting a new code.")
	}

	// generate new code and update pending account
	verificationCode, twerr := accounts.GenerateNewRegistrationCode(ctx, tx, &pendingAccount)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = accounts.SendAccountVerificationCode(pendingAccount.Email, pendingAccount.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.SendNewRegistrationCode: sending email", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("accounts.SendNewRegistrationCode: committing transaction", rz.Err(err))
		return ret, twirp.InternalError(accounts.ErrorSendingNewRegistrationCode)
	}

	return ret, nil
}
