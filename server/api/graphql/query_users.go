package graphql

import (
	"context"
)

func (resolver *queryResolver) Users(ctx context.Context, limit *int, offset *int) ([]*User, error) {
	panic("not implemented")
}
