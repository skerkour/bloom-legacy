package graphql

import (
	"context"
)

func (r *queryResolver) Group(ctx context.Context, id string) (*Group, error) {
	return &Group{
		ID: "lol",
	}, nil
}
