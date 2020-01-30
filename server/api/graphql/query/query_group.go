package query

import (
	"context"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

func (r *Resolver) Group(ctx context.Context, id string) (*model.Group, error) {
	return &model.Group{
		ID: "lol",
	}, nil
}
