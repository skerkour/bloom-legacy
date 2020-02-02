package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (resolver *Resolver) SendNewRegistrationCode(ctx context.Context, input model.SendNewRegistrationCodeInput) (bool, error) {
	ret := false
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("mutaiton.SendNewRegistrationCode: generating random int", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorSendingNewRegistrationCode))
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutaiton.SendNewRegistrationCode: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	var pendingUser users.PendingUser
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", input.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("mutaiton.SendNewRegistrationCode: getting pending user", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	now := time.Now().UTC()
	since := now.Sub(pendingUser.UpdatedAt)
	if since <= 30*time.Second {
		return ret, gqlerrors.New(errors.New(errors.PermissionDenied, "Please wait at least 30 seconds before requesting a new code."))
	}

	// generate new code and update pending user
	verificationCode, err := users.GenerateNewRegistrationCode(ctx, tx, &pendingUser)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = users.SendUserVerificationCode(pendingUser.Email, pendingUser.DisplayName, verificationCode)
	if err != nil {
		tx.Rollback()
		logger.Error("mutaiton.SendNewRegistrationCode: sending email", rz.Err(err))
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutaiton.SendNewRegistrationCode: committing transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	ret = true
	return ret, nil
}
