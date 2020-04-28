package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

type PushParams struct {
	Repositories []RepositoryPush
}

type RepositoryPush struct {
	CurrentState string
	Objects      []APIObject
	GroupID      *uuid.UUID
}

type PushResult struct {
	Repositories []RepositoryPushResult
}

type RepositoryPushResult struct {
	NewState string
	GroupID  *uuid.UUID
}

// Push is used to push changes
func Push(ctx context.Context, actor *users.User, params PushParams) (err error) {
	return
}
