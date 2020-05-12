package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/graphql"
	"gitlab.com/bloom42/lily/uuid"
)

func push() error {
	var err error
	client := api.Client()
	var storedObjects []Object
	objectsToPush := map[string][]Object{}
	var masterKey []byte
	ctx := context.Background()

	tx, err := db.DB.Beginx()
	if err != nil {
		return err
	}

	input := model.PushInput{
		Repositories: []*model.RepositoryPushInput{},
	}

	// find objects that need to be pushed
	storedObjects, err = FindOutOfSyncObjects(ctx, tx)
	if err != nil {
		tx.Rollback()
		return err
	}

	for _, object := range storedObjects {
		var groupID string

		if object.GroupID != nil {
			groupID = object.GroupID.String()
		}
		objectsToPush[groupID] = append(objectsToPush[groupID], object)
	}

	masterKey, err = users.FindMasterKey(ctx, tx)
	if err != nil {
		tx.Rollback()
		return err
	}
	defer crypto.Zeroize(masterKey) // clear masterKey from memory

	// format and encrypt objects
	currentStates.mutex.RLock()
	for groupIDStr, state := range currentStates.states {
		var groupID *uuid.UUID
		objectsPushInput := []*model.ObjectInput{}

		if groupIDStr != "" {
			var groupUUID uuid.UUID

			groupUUID, err = uuid.Parse(groupIDStr)
			if err != nil {
				tx.Rollback()
				return err
			}
			groupID = &groupUUID
		}
		for _, object := range objectsToPush[groupIDStr] {
			var objectToPush *model.ObjectInput

			objectToPush, err = encryptObject(object, masterKey, compressAlgoSnappy)
			if err != nil {
				tx.Rollback()
				return err
			}
			objectsPushInput = append(objectsPushInput, objectToPush)
		}
		repo := &model.RepositoryPushInput{
			CurrentState: state,
			GroupID:      groupID,
			Objects:      objectsPushInput,
		}
		input.Repositories = append(input.Repositories, repo)
	}
	currentStates.mutex.RUnlock()

	var resp struct {
		Push *model.Push `json:"push"`
	}
	req := graphql.NewRequest(`
		mutation ($input: PushInput!) {
			push(input: $input) {
				repositories {
					oldState
					newState
					groupId
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)
	if err != nil {
		tx.Rollback()
		return err
	}

	currentStates.mutex.Lock()
	for _, repo := range resp.Push.Repositories {
		if repo.GroupID != nil {
			currentStates.states[repo.GroupID.String()] = repo.NewState
		} else {
			currentStates.states[""] = repo.NewState
		}
	}
	currentStates.mutex.Unlock()

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		return err
	}

	return nil
}
