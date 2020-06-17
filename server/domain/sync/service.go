package sync

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

// Service is the application service for the sync subdomain
type Service interface {
	// Commands
	Push(ctx context.Context, params PushParams) (ret PushResult, err error)

	// Queries
	Pull(ctx context.Context, params PullParams) (ret PullResult, err error)
}

// PullParams are the parameters for Pull
type PullParams struct {
	Repositories []RepositoryPull
}

type RepositoryPull struct {
	SinceState    string
	SinceStateInt int64
	GroupID       *uuid.UUID
	Group         *groups.Group
}

type PullResult struct {
	Repositories []RepositoryPullResult
}

type RepositoryPullResult struct {
	OldState       string
	NewState       string
	HasMoreChanges bool
	Objects        []Object
	GroupID        *uuid.UUID
}

// PushParams are the parameters for Push
type PushParams struct {
	Repositories []RepositoryPush
}

type RepositoryPush struct {
	CurrentState   string
	CurentStateInt int64
	Objects        []APIObject
	GroupID        *uuid.UUID
}

type PushResult struct {
	Repositories []RepositoryPushResult
}

type RepositoryPushResult struct {
	OldState string
	NewState string
	GroupID  *uuid.UUID
}
