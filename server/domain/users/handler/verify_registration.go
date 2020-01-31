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

func (s Handler) VerifyRegistration(ctx context.Context, params *rpc.VerifyRegistrationParams) (*rpc.Empty, error) {
	ret := &rpc.Empty{}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.VerifyRegistration: error getting apiCtx from context")
		return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
	}
	if apiCtx.AuthenticatedUser != nil {
		twerr := twirp.NewError(twirp.PermissionDenied, "Must not be authenticated")
		return ret, twerr
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.VerifyRegistration: generating random int", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// verify pending user
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.VerifyRegistration: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
	}

	var pendingUser users.PendingUser
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", params.Id)
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyRegistration: getting pending user", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
	}

	twerr := users.VerifyPendingUser(ctx, tx, pendingUser, params.Code)
	if twerr != nil {
		tx.Rollback()
		tx, _ := db.DB.Beginx()
		if tx != nil {
			twerr2 := users.FailPendingUserVerification(ctx, tx, pendingUser)
			if twerr2 != nil {
				tx.Rollback()
				return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
			}
			tx.Commit()
		}
		return ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.VerifyRegistration: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(users.ErrorVerifyPendingUserMsg)
	}

	return ret, nil
}
