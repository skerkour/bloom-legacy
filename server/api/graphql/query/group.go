package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// Group find a group
func (r *Resolver) Group(ctx context.Context, id string) (*model.Group, error) {
	panic("not implemented")
}
