package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

func (r *Resolver) CompletePasswordUpdate(ctx context.Context, input model.CompletePasswordUpdateInput) (*model.SignedIn, error) {
	return nil, nil
}
