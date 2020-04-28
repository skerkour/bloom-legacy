package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// PushParams are the parameters for Push
type PushParams struct {
	Repositories []RepositoryPush
}

type RepositoryPush struct {
	CurrentState   string
	curentStateInt int64
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

// Push is used to push changes
func Push(ctx context.Context, actor *users.User, params PushParams) (ret *PushResult, err error) {
	logger := rz.FromCtx(ctx)
	ret = &PushResult{Repositories: []RepositoryPushResult{}}

	// cleant and validate params
	for i, repo := range params.Repositories {
		var curentState int64
		curentState, err = DecodeStateString(repo.CurrentState)
		if err != nil {
			return
		}
		params.Repositories[i].curentStateInt = curentState
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("objects.Push: Starting transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}

	for _, repo := range params.Repositories {
		var result RepositoryPushResult
		result, err = pushToRepository(ctx, tx, actor, &repo)
		if err != nil {
			tx.Rollback()
			return
		}
		ret.Repositories = append(ret.Repositories, result)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("objects.Push: Committing transaction", rz.Err(err))
		err = NewError(ErrorInternal)
		return
	}

	return
}

func pushToRepository(ctx context.Context, tx *sqlx.Tx, actor *users.User, repo *RepositoryPush) (ret RepositoryPushResult, err error) {
	if repo.GroupID != nil {
		// check if user is group member
		// for each object, check if it exists, if yes, if it belongs to group
		// update object
		// else insert object

	} else {
		if actor.State != repo.curentStateInt {
			err = NewError(ErrorOutOfSync)
			return
		}
		// for each object, check if it exists, if yes, if it belongs to user
		// update object
		// else insert object
	}

	newState := repo.curentStateInt + 1
	ret.NewState = EncodeState(newState)
	ret.OldState = repo.CurrentState
	ret.GroupID = repo.GroupID
	return
}
