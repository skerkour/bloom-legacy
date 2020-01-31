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

func (s Handler) CompleteRegistration(ctx context.Context, params *rpc.CompleteRegistrationParams) (*rpc.NewSession, error) {
	var ret rpc.NewSession
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.CompleteRegistration: error getting apiCtx from context")
		return &ret, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}
	if apiCtx.AuthenticatedUser != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return &ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.CompleteRegistration: generating random int", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorCreatePendingUserMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.CompleteRegistration: Starting transaction", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorCompletingRegistrationMsg)
	}

	// find pending user
	var pendingUser users.PendingUser
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("users.CompleteRegistration: getting pending user", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorCompletingRegistrationMsg)
	}

	// delete pending user
	twerr := users.DeletePendingUser(ctx, tx, pendingUser.ID)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	// create user
	newUser, twerr := users.CreateUser(ctx, tx, pendingUser, params.Username, params.AuthKey)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	// start session
	newSessiont, token, twerr := users.StartSession(ctx, tx, newUser.ID, apiCtx.IP, apiCtx.UserAgent)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyRegistration: Committing transaction", rz.Err(err))
		return &ret, twirp.InternalError(users.ErrorCompletingRegistrationMsg)
	}

	ret = rpc.NewSession{
		Id:    newSessiont.ID,
		Token: token,
	}
	return &ret, nil
}
