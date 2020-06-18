package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// StartPasswordUpdate initiate a password update
func (resolver *Resolver) StartPasswordUpdate(ctx context.Context) (*model.PasswordUpdateStarted, error) {
	return nil, nil
}
