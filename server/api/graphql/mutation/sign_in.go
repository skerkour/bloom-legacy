package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/api/graphql/errors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) SignIn(ctx context.Context, input model.SignInInput) (*model.SignedIn, error) {
	var ret model.SignedIn
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		logger.Error("users.SignIn: error getting apiCtx from context")
		return &ret, errors.New(errors.Internal, users.ErrorSingingInMsg)
	}
	if apiCtx.AuthenticatedUser != nil {
		return &ret, errors.New(errors.PermissionDenied, "Must not be authenticated")
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("users.SignIn: generating random int", rz.Err(err))
		return &ret, errors.New(errors.Internal, users.ErrorSingingInMsg)
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.SignIn: Starting transaction", rz.Err(err))
		return &ret, errors.New(errors.Internal, users.ErrorSingingInMsg)
	}

	// fetch user
	var user users.User
	err = tx.Get(&user, "SELECT * FROM users WHERE username = $1 FOR UPDATE", input.Username)
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignIn: finding user", rz.Err(err))
		return &ret, errors.New(errors.PermissionDenied, "Invalid Username / Password combination")
	}

	// verify password
	if !argon2id.VerifyPassword(input.AuthKey, user.AuthKeyHash) {
		tx.Rollback()
		return &ret, errors.New(errors.PermissionDenied, "Invalid Username / Password combination")
	}

	newSession, token, twerr := users.StartSession(ctx, tx, user.ID, apiCtx.IP, apiCtx.UserAgent)
	if twerr != nil {
		tx.Rollback()
		return &ret, twerr
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.SignIn: committing transaction", rz.Err(err))
		return &ret, errors.New(errors.Internal, users.ErrorSingingInMsg)
	}

	ret = model.SignedIn{
		Session: &model.Session{
			ID:     newSession.ID,
			Token:  &token,
			Device: nil,
		},
	}
	return &ret, nil
}
