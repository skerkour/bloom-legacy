package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/http/httputil"
)

// SignIn is used to sign-in, or initiate a pending session if 2fa is enabled
func (resolver *Resolver) SignIn(ctx context.Context, input model.SignInInput) (ret *model.SignedIn, err error) {
	httpCtx := httputil.HTTPCtxFromCtx(ctx)

	params := users.SignInParams{
		Username: input.Username,
		AuthKey:  input.AuthKey,
		Device: users.SessionDevice{
			OS:   input.Device.Os.String(),
			Type: input.Device.Type.String(),
		},
		IPAddress: httpCtx.IP,
	}
	user, session, token, err := resolver.usersService.SignIn(ctx, params)
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
