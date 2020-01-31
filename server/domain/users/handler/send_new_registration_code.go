package handler

import (
	"context"
	"time"

	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) SendNewRegistrationCode(ctx context.Context, params *rpc.SendNewRegistrationCodeParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.SendNewRegistrationCode: error getting apiCtx from context")
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}
	if apiCtx.AuthenticatedUser != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.SendNewRegistrationCode: generating random int", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SendNewRegistrationCode: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}

	var pendingUser users.PendingUser
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SendNewRegistrationCode: getting pending user", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since <= 30*time.Second {
		return ret, twirp.NewError(twirp.PermissionDenied, "Please wait at least 30 seconds before requesting a new code.")
	}

	// generate new code and update pending user
	verificationCode, twerr := users.GenerateNewRegistrationCode(ctx, tx, &pendingUser)
	if twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	err = users.SendUserVerificationCode(pendingUser.Email, pendingUser.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SendNewRegistrationCode: sending email", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.SendNewRegistrationCode: committing transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorSendingNewRegistrationCode)
	}

	return ret, nil
}
