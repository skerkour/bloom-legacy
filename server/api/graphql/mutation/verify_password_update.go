package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// VerifyPasswordUpdate is used to verify a password update code received by email
func (resolver *Resolver) VerifyPasswordUpdate(ctx context.Context, input model.VerifyPasswordUpdateInput) (bool, error) {
	return false, nil
}
