package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/crypto42-go/rand"
	"gitlab.com/bloom42/libs/rz-go"
)

func (r *Resolver) SignIn(ctx context.Context, input model.SignInInput) (*model.SignedIn, error) {
	var ret *model.SignedIn
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)
	apiCtx := apiutil.ApiCtxFromCtx(ctx)
	if apiCtx == nil {
		logger.Error("mutation.SignIn: error getting apiCtx from context")
		return ret, gqlerrors.Internal()
	}

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := rand.Int64(500, 800)
	if err != nil {
		logger.Error("mutation.SignIn: generating random int", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.SignIn: Starting transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
	}

	// fetch user
	user, err := users.FindUserByUsername(ctx, tx, input.Username)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(users.NewError(users.ErrorInvalidUsernamePasswordCombination))
	}

	// verify password
	if !argon2id.VerifyPassword(input.AuthKey, user.AuthKeyHash) {
		tx.Rollback()
		return ret, gqlerrors.New(users.NewError(users.ErrorInvalidUsernamePasswordCombination))
	}

	device := users.SessionDevice{
		OS:   input.Device.Os.String(),
		Type: input.Device.Type.String(),
	}

	newSession, token, err := users.StartSession(ctx, tx, user.ID, device)
	if err != nil {
		tx.Rollback()
		return ret, gqlerrors.New(err)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.SignIn: committing transaction", rz.Err(err))
		return ret, gqlerrors.New(users.NewError(users.ErrorSingingIn))
	}

	// TODO: send login email

	ret = &model.SignedIn{
		Session: &model.Session{
			ID:    newSession.ID,
			Token: &token,
			Device: &model.SessionDevice{
				Os:   model.SessionDeviceOs(device.OS),
				Type: model.SessionDeviceType(device.Type),
			},
		},
		Me: &model.User{
			ID:          &user.ID,
			AvatarURL:   nil,
			CreatedAt:   &user.CreatedAt,
			Username:    user.Username,
			FirstName:   &user.FirstName,
			LastName:    &user.LastName,
			DisplayName: user.DisplayName,
			IsAdmin:     user.IsAdmin,
		},
	}
	return ret, nil
}
