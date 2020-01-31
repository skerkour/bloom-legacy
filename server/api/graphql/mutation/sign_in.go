package mutation

import (
	"context"
	"time"

	gqlerrors "gitlab.com/bloom42/bloom/server/api/graphql/errors"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) SignIn(ctx context.Context, input model.SignInInput) (*model.SignedIn, error) {
	var ret model.SignedIn
	logger := rz.FromCtx(ctx)
	currentUser := gqlutil.UserFromCtx(ctx)
	apiCtx := gqlutil.ApiCtxFromCtx(ctx)
	if apiCtx == nil {
		logger.Error("mutation.SignIn: error getting apiCtx from context")
		return &ret, gqlerrors.Internal()
	}

	if currentUser != nil {
		return &ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("mutation.SignIn: generating random int", rz.Err(err))
		return &ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.SignIn: Starting transaction", rz.Err(err))
		return &ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
	}

	// fetch user
	var user users.User
	err = tx.Get(&user, "SELECT * FROM users WHERE username = $1 FOR UPDATE", input.Username)
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.SignIn: finding user", rz.Err(err))
		return &ret, gqlerrors.New(errors.New(errors.PermissionDenied, "Invalid Username / Password combination"))
	}

	// verify password
	if !argon2id.VerifyPassword(input.AuthKey, user.AuthKeyHash) {
		tx.Rollback()
		return &ret, gqlerrors.New(errors.New(errors.PermissionDenied, "Invalid Username / Password combination"))
	}

	device := users.SessionDevice{
		OS:   input.Device.Os.String(),
		Type: input.Device.Type.String(),
	}

	newSession, token, err := users.StartSession(ctx, tx, user.ID, apiCtx.IP, apiCtx.UserAgent, device)
	if err != nil {
		tx.Rollback()
		return &ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.SignIn: committing transaction", rz.Err(err))
		return &ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
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
