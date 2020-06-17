package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// CompleteRegistration is used to complete an account's registration
func (resolver *Resolver) CompleteRegistration(ctx context.Context, input model.CompleteRegistrationInput) (ret *model.SignedIn, err error) {
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
	user, session, token, err := resolver.usersService.CompleteRegistration(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.SignedIn{
		Session: &model.Session{
			ID:    session.ID,
			Token: &token,
			Device: &model.SessionDevice{
				Os:   model.SessionDeviceOs(session.DeviceOS),
				Type: model.SessionDeviceType(session.DeviceType),
			},
		},
		Me: model.DomainUserToModelUser(user, user),
	}
	return
}
