package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
)

func (service *SyncService) Push(ctx context.Context, params sync.PushParams) (ret sync.PushResult, err error) {
	ret = sync.PushResult{
		Repositories: []sync.RepositoryPushResult{},
	}
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate params
	for i, repo := range params.Repositories {
		var curentState int64
		curentState, err = sync.DecodeStateString(repo.CurrentState)
		if err != nil {
			return
		}
		params.Repositories[i].CurentStateInt = curentState
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "sync.Push: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	for _, repo := range params.Repositories {
		var result sync.RepositoryPushResult

		result, err = service.pushToRepository(ctx, tx, me, repo)
		if err != nil {
			tx.Rollback()
			return
		}
		ret.Repositories = append(ret.Repositories, result)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "sync.Push: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}

func (service *SyncService) pushToRepository(ctx context.Context, db db.Queryer, actor users.User, repo sync.RepositoryPush) (ret sync.RepositoryPushResult, err error) {
	newState := repo.CurentStateInt + 1
	var pushSize int64
	var customer billing.Customer

	if repo.GroupID != nil {
		group, err := service.groupsRepo.FindGroupByID(ctx, db, *repo.GroupID)
		if err != nil {
			return ret, err
		}

		// check if user is group member
		err = service.groupsService.CheckUserIsGroupMember(ctx, db, actor.ID, group.ID)
		if err != nil {
			return ret, err
		}

		customer, err = service.billingRepo.FindCustomerByGroupID(ctx, db, group.ID)
		if err != nil {
			return ret, err
		}

		// for each object, check if it exists, if yes, if it belongs to group
		// update object
		// else insert object
		for _, repoObject := range repo.Objects {
			// take in account the tag
			if len(repoObject.EncryptedKey) != 0 && len(repoObject.EncryptedKey) > crypto.KeySize256+100 {
				err = sync.ErrInvalidObjectKeySize
				return ret, err
			}
			if len(repoObject.Nonce) != 0 && len(repoObject.Nonce) != crypto.AEADNonceSize {
				err = sync.ErrInvalidObjectNonceSize
				return ret, err
			}

			object, err := service.syncRepo.FindObjectByID(ctx, db, repoObject.ID)
			if err != nil {
				if _, ok := err.(*errors.NotFoundError); !ok {
					return ret, err
				}
			}

			// object not found
			if err != nil {
				// insert object
				object.Algorithm = repoObject.Algorithm
				object.Nonce = repoObject.Nonce
				object.EncryptedKey = repoObject.EncryptedKey
				object.EncryptedData = repoObject.EncryptedData
				object.UpdatedAtState = newState
				object.ID = repoObject.ID
				object.GroupID = repo.GroupID

				pushSize += int64(len(object.EncryptedData))
				err = service.syncRepo.CreateObject(ctx, db, object)
				if err != nil {
					return ret, err
				}
			} else {
				// check if it belongs to user
				if object.GroupID == nil || *object.GroupID != *repo.GroupID {
					err = sync.ErrObjectNotFound
					return ret, err
				}

				// update object
				pushSize -= int64(len(object.EncryptedData))

				object.Algorithm = repoObject.Algorithm
				object.Nonce = repoObject.Nonce
				object.EncryptedKey = repoObject.EncryptedKey
				object.EncryptedData = repoObject.EncryptedData
				object.UpdatedAtState = newState

				pushSize += int64(len(object.EncryptedData))
				err = service.syncRepo.UpdateObject(ctx, db, object)
				if err != nil {
					return ret, err
				}
			}
		}
		group.State = newState
		err = service.groupsRepo.UpdateGroup(ctx, db, group)
		if err != nil {
			return ret, err
		}
		_, err = service.billingService.UpdateCustomerUsedStorage(ctx, db, customer, pushSize)
		if err != nil {
			return ret, err
		}

	} else {
		// user's repository
		if actor.State != repo.CurentStateInt {
			err = sync.ErrOutOfSync
			return ret, err
		}

		customer, err := service.billingRepo.FindCustomerByUserID(ctx, db, actor.ID)
		if err != nil {
			return ret, err
		}

		for _, repoObject := range repo.Objects {
			// take in account the tag
			if len(repoObject.EncryptedKey) != 0 && len(repoObject.EncryptedKey) > crypto.KeySize256+100 {
				err = sync.ErrInvalidObjectKeySize
				return ret, err
			}
			if len(repoObject.Nonce) != 0 && len(repoObject.Nonce) != crypto.AEADNonceSize {
				err = sync.ErrInvalidObjectNonceSize
				return ret, err
			}

			object, err := service.syncRepo.FindObjectByID(ctx, db, repoObject.ID)
			if err != nil {
				if _, ok := err.(*errors.NotFoundError); !ok {
					return ret, err
				}
			}
			// object not found
			if err != nil {
				// insert object
				object.Algorithm = repoObject.Algorithm
				object.Nonce = repoObject.Nonce
				object.EncryptedKey = repoObject.EncryptedKey
				object.EncryptedData = repoObject.EncryptedData
				object.UpdatedAtState = newState
				object.ID = repoObject.ID
				object.UserID = &actor.ID

				pushSize += int64(len(object.EncryptedData))

				err = service.syncRepo.CreateObject(ctx, db, object)
				if err != nil {
					return ret, err
				}
			} else {
				// check if it belongs to user
				if object.UserID == nil || *object.UserID != actor.ID {
					err = sync.ErrObjectNotFound
					return ret, err
				}

				// update object
				pushSize -= int64(len(object.EncryptedData))

				object.Algorithm = repoObject.Algorithm
				object.Nonce = repoObject.Nonce
				object.EncryptedKey = repoObject.EncryptedKey
				object.EncryptedData = repoObject.EncryptedData
				object.UpdatedAtState = newState

				pushSize += int64(len(object.EncryptedData))

				err = service.syncRepo.UpdateObject(ctx, db, object)
				if err != nil {
					return ret, err
				}
			}
		}

		actor.State = newState
		err = service.usersRepo.UpdateUser(ctx, db, actor)
		if err != nil {
			return ret, err
		}
		_, err = service.billingService.UpdateCustomerUsedStorage(ctx, db, customer, pushSize)
		if err != nil {
			return ret, err
		}
	}

	ret.NewState = sync.EncodeState(newState)
	ret.OldState = repo.CurrentState
	ret.GroupID = repo.GroupID
	return
}
