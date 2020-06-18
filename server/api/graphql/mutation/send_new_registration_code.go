package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// SendNewRegistrationCode is used to send a new pending user code by email
func (resolver *Resolver) SendNewRegistrationCode(ctx context.Context, input model.SendNewRegistrationCodeInput) (ret bool, err error) {
	err = resolver.usersService.SendNewRegistrationCode(ctx, input.ID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
