package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Pull returns the changes from a given state
func (resolver *Resolver) Pull(ctx context.Context, input model.PullInput) (*model.Pull, error) {
	panic("not implemented")
	return nil, nil
}
