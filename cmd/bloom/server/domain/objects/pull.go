package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

// PullParams are the parameters for Pull
type PullParams struct {
	Repositories []RepositoryPull
}

type RepositoryPull struct {
	SinceState string
	GroupID    *uuid.UUID
}

type PullResult struct {
	Repositories []RepositoryPullResult
}

type RepositoryPullResult struct {
	OldState       string
	NewState       string
	HasMoreChanges bool
	Objects        []APIObject
	GroupID        *uuid.UUID
}

// Pull is used to pull changes
func Pull(ctx context.Context, actor *users.User, params PullParams) (ret *PullResult, err error) {
	ret = &PullResult{Repositories: []RepositoryPullResult{}}
	return
}
