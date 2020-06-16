package mutation

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/bloom/server/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/rz"
)

// CompleteRegistration is used to complete an account's registration
func (r *Resolver) CompleteRegistration(ctx context.Context, input model.CompleteRegistrationInput) (ret *model.SignedIn, err error) {
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)
	apiCtx := apiutil.ApiCtxFromCtx(ctx)
	if apiCtx == nil {
		logger.Error("mutation.CompleteRegistration: error getting apiCtx from context")
		err = gqlerrors.Internal()
		return
	}

	if currentUser != nil {
		return ret, gqlerrors.MustNotBeAuthenticated()
	}

	// sleep to prevent spam and bruteforce
	sleep, err := crypto.RandInt64(500, 800)
	if err != nil {
		logger.Error("mutation.CompleteRegistration: generating random int", rz.Err(err))
		err = gqlerrors.New(users.NewError(users.ErrorCompletingRegistration))
		return
	}
	time.Sleep(time.Duration(sleep) * time.Millisecond)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CompleteRegistration: Starting transaction", rz.Err(err))
		err = gqlerrors.New(users.NewError(users.ErrorCompletingRegistration))
		return
	}

	params := users.CompleteRegistrationParams{
		PendingUserID:       input.ID,
		Username:            input.Username,
		AuthKey:             input.AuthKey,
		PublicKey:           input.PublicKey,
		EncryptedPrivateKey: input.EncryptedPrivateKey,
		PrivateKeyNonce:     input.PrivateKeyNonce,
		EncryptedMasterKey:  input.EncryptedMasterKey,
		MasterKeyNonce:      input.MasterKeyNonce,
		Device: users.SessionDevice{
			OS:   input.Device.Os.String(),
			Type: input.Device.Type.String(),
		},
	}
	newUser, newSession, token, err := users.CompleteRegistration(ctx, tx, params)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, newUser, &newUser.ID, nil)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CompleteRegistration: Committing transaction", rz.Err(err))
		err = gqlerrors.Internal()
		return
	}

	ret = &model.SignedIn{
		Session: &model.Session{
			ID:    newSession.ID,
			Token: &token,
			Device: &model.SessionDevice{
				Os:   model.SessionDeviceOs(newSession.DeviceOS),
				Type: model.SessionDeviceType(newSession.DeviceType),
			},
		},
		Me: &model.User{
			ID:          &newUser.ID,
			AvatarURL:   nil,
			CreatedAt:   &newUser.CreatedAt,
			Username:    newUser.Username,
			FirstName:   &newUser.FirstName,
			LastName:    &newUser.LastName,
			DisplayName: newUser.DisplayName,
			IsAdmin:     newUser.IsAdmin,
		},
	}
	return ret, nil
}
