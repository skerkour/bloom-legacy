package query

import (
	"context"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (resolver *Resolver) Users(ctx context.Context, limit *int, offset *int) ([]*model.User, error) {
	panic("not implemented")
}
