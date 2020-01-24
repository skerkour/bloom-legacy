package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/users"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (s Handler) StartRegistration(ctx context.Context, params *rpc.StartRegistrationParams) (*rpc.RegistrationStarted, error) {
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.StartRegistration: error getting apiCtx from context")
		return &rpc.RegistrationStarted{}, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}
	if apiCtx.AuthenticatedUser != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &rpc.RegistrationStarted{}, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.StartRegistration: generating random int", rz.Err(err))
		return &rpc.RegistrationStarted{}, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// create pending user
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.StartRegistration: Starting transaction", rz.Err(err))
		return &rpc.RegistrationStarted{}, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}

	newPendingUser, verificationCode, twerr := users.CreatePendingUser(ctx, tx, params.DisplayName, params.Email)
	if twerr != nil {
		tx.Rollback()
		return &rpc.RegistrationStarted{}, twerr
	}

	err = users.SendUserVerificationCode(params.Email, params.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("users.StartRegistration: Sending confirmation email", rz.Err(err))
		return &rpc.RegistrationStarted{}, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.StartRegistration: Committing transaction", rz.Err(err))
		return &rpc.RegistrationStarted{}, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}

	ret := rpc.RegistrationStarted{
		Id: newPendingUser.ID,
	}
	return &ret, nil
}
