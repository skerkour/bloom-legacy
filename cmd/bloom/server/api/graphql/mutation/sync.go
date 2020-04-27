package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

func (resolver *Resolver) Sync(ctx context.Context, input model.SyncInput) (*model.Sync, error) {
	return nil, nil
}
