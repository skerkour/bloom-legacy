package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// Users finds all users
func (resolver *Resolver) Users(ctx context.Context) (*model.UserConnection, error) {
	panic("not implemented")
}
