package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (resolver *Resolver) VerifyRegistration(ctx context.Context, input model.VerifyRegistrationInput) (bool, error) {
	ret := false
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("mutation.VerifyRegistration: generating random int", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	// verify pending user
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.VerifyRegistration: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	var pendingUser users.PendingUser
	err = tx.Get(&pendingUser, "SELECT * FROM pending_users WHERE id = $1 FOR UPDATE", input.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.VerifyRegistration: getting pending user", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	err = users.VerifyPendingUser(ctx, tx, &pendingUser, input.Code)
	if err != nil {
		tx.Rollback()
		tx, _ := db.DB.Beginx()
		if tx != nil {
			err2 := users.FailPendingUserVerification(ctx, tx, pendingUser)
			if err2 != nil {
				tx.Rollback()
				return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
			}
			tx.Commit()
		}
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.VerifyRegistration: Committing transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorVerifyingPendingUser))
	}

	ret = true
	return ret, nil
}
