package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

func (r *Resolver) CompleteSignIn(ctx context.Context, input model.CompleteSignInInput) (*model.SignedIn, error) {
	return nil, nil
}
