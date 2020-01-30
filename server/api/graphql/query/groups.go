package query

import (
	"context"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (r *Resolver) Groups(ctx context.Context, limit *int, offset *int) ([]*model.Group, error) {
	panic("not implemented")
}
