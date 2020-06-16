package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
)

// StartTwoFAActivation starts the activation of 2fa for an user
func (resolver *Resolver) StartTwoFAActivation(ctx context.Context) (*model.TwoFAActivationStarted, error) {
	panic("not implemented")
	return nil, nil
}
