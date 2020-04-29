package sync

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// PullParams are the parameters for Pull
type PullParams struct {
	Repositories []RepositoryPull
}

type RepositoryPull struct {
	SinceState    string
	sinceStateInt int64
	GroupID       *uuid.UUID
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

// Pull is used to pull changes
func Pull(ctx context.Context, actor *users.User, params PullParams) (ret *PullResult, err error) {
	ret = &PullResult{Repositories: []RepositoryPullResult{}}
	logger := rz.FromCtx(ctx)

	// clean and validate params
	for i, repo := range params.Repositories {
		var sinceStateInt int64
		sinceStateInt, err = DecodeStateString(repo.SinceState)
		if err != nil {
			err = NewError(ErrorInternal)
			return
		}
		params.Repositories[i].sinceStateInt = sinceStateInt
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("objects.Pull: Starting transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}

	for _, repo := range params.Repositories {
		var result RepositoryPullResult
		result, err = pullRepository(ctx, tx, actor, &repo)
		if err != nil {
			tx.Rollback()
			return
		}
		ret.Repositories = append(ret.Repositories, result)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("objects.Pull: Committing transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}
	return
}

func pullRepository(ctx context.Context, tx *sqlx.Tx, actor *users.User, repo *RepositoryPull) (ret RepositoryPullResult, err error) {
	ret.Objects = []APIObject{}
	ret.OldState = repo.SinceState

	if repo.GroupID != nil {
		var group *groups.Group
		var objects []Object

		group, err = groups.FindGroupById(ctx, tx, *repo.GroupID, false)
		if err != nil {
			return
		}
		if repo.sinceStateInt == group.State {
			ret.NewState = EncodeState(group.State)
			return
		}
		objects, err = FindObjectSinceState(ctx, tx, repo.sinceStateInt, nil, &group.ID)
		if err != nil {
			return
		}
		ret.NewState = EncodeState(group.State)
		ret.Objects = objects

	} else {
		// user's repository
		var objects []Object

		if repo.sinceStateInt == actor.State {
			ret.NewState = EncodeState(actor.State)
			return
		}

		objects, err = FindObjectSinceState(ctx, tx, repo.sinceStateInt, &actor.ID, nil)
		if err != nil {
			return
		}
		ret.NewState = EncodeState(actor.State)
		ret.Objects = objects
	}
	return
}
