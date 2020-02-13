package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// User finds an user
func (resolver *Resolver) User(ctx context.Context, username *string) (*model.User, error) {
	panic("not implemented")
}
