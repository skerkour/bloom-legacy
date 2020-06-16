package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// CompletePasswordUpdate is used to complete a password update
func (r *Resolver) CompletePasswordUpdate(ctx context.Context, input model.CompletePasswordUpdateInput) (*model.SignedIn, error) {
	panic("not implemented")
	return nil, nil
}
