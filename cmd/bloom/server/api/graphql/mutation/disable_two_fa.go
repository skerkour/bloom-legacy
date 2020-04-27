package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// DisableTwoFa disables 2fa for an user
func (resolver *Resolver) DisableTwoFa(ctx context.Context, input model.DisableTwoFAInput) (ret bool, err error) {
	panic("not implemented")
	return
}
