package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

// Push is used to push changes
func (resolver *Resolver) Push(ctx context.Context, input model.PushInput) (*model.Push, error) {
	panic("not implemented")
	return nil, nil
}
