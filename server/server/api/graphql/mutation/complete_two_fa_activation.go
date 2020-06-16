package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
)

// CompleteTwoFAActivation starts the activation of 2fa for an user
func (resolver *Resolver) CompleteTwoFAActivation(ctx context.Context, input model.CompleteTwoFAActivationInput) (ret bool, err error) {
	panic("not implemented")
	return
}
