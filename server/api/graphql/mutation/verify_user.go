package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/users"
)

// VerifyUser is used to verify a pending user code received by email
func (resolver *Resolver) VerifyUser(ctx context.Context, input model.VerifyUserInput) (ret bool, err error) {
	params := users.VerifyPendingUserParams{
		PendingUserID: input.ID,
		Code:          input.Code,
	}
	err = resolver.usersService.VerifyPendingUser(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
