package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
)

func (resolver *Resolver) VerifyPasswordUpdate(ctx context.Context, input model.VerifyPasswordUpdateInput) (bool, error) {
	return false, nil
}
