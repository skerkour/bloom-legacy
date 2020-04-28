package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
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
	newState := repo.curentStateInt + 1

	if repo.GroupID != nil {
		// check if user is group member
		err = groups.CheckUserIsGroupMember(ctx, tx, actor.ID, *repo.GroupID)
		if err != nil {
			return
		}
		// for each object, check if it exists, if yes, if it belongs to group
		// update object
		// else insert object
		for _, repoObject := range repo.Objects {
			var object *Object
			object, err = FindObjectByID(ctx, tx, repoObject.ID, true)
			if err != nil {
				return
			}

			object.Algorithm = repoObject.Algorithm
			object.Nonce = repoObject.Nonce
			object.EncryptedKey = repoObject.EncryptedKey
			object.EncryptedData = repoObject.EncryptedData
			object.UpdatedAtState = newState

			if object == nil {
				// insert object
				object.ID = repoObject.ID
				object.GroupID = repo.GroupID
				queryInsert := `INSERT INTO obejcts
					(id, updated_at_state, algorithm, nonce, encrypted_key, encrypted_data, group_id)
					VALUES ($1, $2, $3, $4, $5, $6, $7)
				`
				_, err = tx.Exec(queryInsert, object.ID, object.UpdatedAtState, object.Algorithm,
					object.Nonce, object.EncryptedKey, object.EncryptedData, object.GroupID)
				if err != nil {
					return
				}
			} else {
				// check if it belongs to user
				if object.GroupID == nil || *object.GroupID != *repo.GroupID {
					err = NewError(ErrorObjectNotFound)
					return
				}
				// update object
				queryUpdate := `UPDATE obejcts
					SET algorithm = $1, nonce = $2, encrypted_key = $3, encrypted_data = $4, updated_at_state = $5
					WHERE id = $6
				`
				_, err = tx.Exec(queryUpdate, object.Algorithm, object.Nonce, object.EncryptedKey,
					object.EncryptedData, object.UpdatedAtState, object.ID)
				if err != nil {
					return
				}
			}
		}

	} else {
		if actor.State != repo.curentStateInt {
			err = NewError(ErrorOutOfSync)
			return
		}
		for _, repoObject := range repo.Objects {
			var object *Object
			object, err = FindObjectByID(ctx, tx, repoObject.ID, true)
			if err != nil {
				return
			}

			object.Algorithm = repoObject.Algorithm
			object.Nonce = repoObject.Nonce
			object.EncryptedKey = repoObject.EncryptedKey
			object.EncryptedData = repoObject.EncryptedData
			object.UpdatedAtState = newState

			if object == nil {
				// insert object
				object.ID = repoObject.ID
				object.UserID = &actor.ID
				queryInsert := `INSERT INTO obejcts
					(id, updated_at_state, algorithm, nonce, encrypted_key, encrypted_data, user_id)
					VALUES ($1, $2, $3, $4, $5, $6, $7)
				`
				_, err = tx.Exec(queryInsert, object.ID, object.UpdatedAtState, object.Algorithm,
					object.Nonce, object.EncryptedKey, object.EncryptedData, object.UserID)
				if err != nil {
					return
				}
			} else {
				// check if it belongs to user
				if object.UserID == nil || *object.UserID != actor.ID {
					err = NewError(ErrorObjectNotFound)
					return
				}
				// update object
				queryUpdate := `UPDATE obejcts
					SET algorithm = $1, nonce = $2, encrypted_key = $3, encrypted_data = $4, updated_at_state = $5
					WHERE id = $6
				`
				_, err = tx.Exec(queryUpdate, object.Algorithm, object.Nonce, object.EncryptedKey,
					object.EncryptedData, object.UpdatedAtState, object.ID)
				if err != nil {
					return
				}
			}
		}
		err = users.UpdateUserState(ctx, tx, actor, newState)
		if err != nil {
			return
		}
	}

	ret.NewState = EncodeState(newState)
	ret.OldState = repo.CurrentState
	ret.GroupID = repo.GroupID
	return
}
