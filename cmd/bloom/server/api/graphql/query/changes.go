package query

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Changes returns the changes from a given state
func (resolver *Resolver) Changes(ctx context.Context, sinceState *string) (*model.Changes, error) {
	panic("not implemented")
	return nil, nil
}
