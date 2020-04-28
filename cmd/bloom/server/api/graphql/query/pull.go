package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Oull returns the changes from a given state
func (resolver *Resolver) Pull(ctx context.Context, sinceState *string) (*model.Pull, error) {
	panic("not implemented")
	return nil, nil
}
