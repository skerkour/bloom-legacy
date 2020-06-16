package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// CompleteSignIn is used to complete a sign-in when 2fa is activated
func (r *Resolver) CompleteSignIn(ctx context.Context, input model.CompleteSignInInput) (*model.SignedIn, error) {
	panic("not implemented")
	return nil, nil
}
