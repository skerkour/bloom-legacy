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

func pull() error {
	var err error
	client := api.Client()
	ctx := context.Background()
	var masterKey []byte

	tx, err := db.DB.Beginx()
	if err != nil {
		return err
	}

	// prepare api request
	input := model.PullInput{
		Repositories: []*model.RepositoryPullInput{},
	}

	currentStates.mutex.RLock()
	for groupIDStr, state := range currentStates.states {
		var groupID *uuid.UUID

		if groupIDStr != "" {
			var groupUUID uuid.UUID
			groupUUID, err = uuid.Parse(groupIDStr)
			if err != nil {
				tx.Rollback()
				return err
			}
			groupID = &groupUUID
		}
		repo := &model.RepositoryPullInput{
			SinceState: state,
			GroupID:    groupID,
		}
		input.Repositories = append(input.Repositories, repo)
	}
	currentStates.mutex.RUnlock()

	// build api request
	var resp struct {
		Pull *model.Pull `json:"pull"`
	}
	req := graphql.NewRequest(`
		mutation ($input: PullInput!) {
			pull(input: $input) {
				repositories {
					oldState
					newState
					objects {
						id
						algorithm
						encryptedData
						encryptedKey
						nonce
					}
					hasMoreChanges
					groupId
				}
			}
		}
	`)
	req.Var("input", input)

	err = client.Do(ctx, req, &resp)
	if err != nil {
		tx.Rollback()
		return err
	}

	masterKey, err = users.FindMasterKey(ctx, tx)
	if err != nil {
		tx.Rollback()
		return err
	}
	defer crypto.Zeroize(masterKey)

	for _, repo := range resp.Pull.Repositories {

		for _, object := range repo.Objects {
			decryptedObject, err := decryptObject(object, masterKey)
			if err != nil {
				tx.Rollback()
				return err
			}
			// check if object exist and is out of sync
			ofsStoredObject, err := FindOutOfSyncObjectByID(ctx, tx, decryptedObject.ID)
			if ofsStoredObject != nil {
				// resolve conflict
				// create a new object from the local out of sync object (with a new id)

			} else {
				// save object
				err = SaveObject(ctx, tx, decryptedObject)
				if err != nil {
					tx.Rollback()
					return err
				}
			}

		}

		currentStates.mutex.Lock()
		if repo.GroupID != nil {
			currentStates.states[repo.GroupID.String()] = repo.NewState
		} else {
			currentStates.states[""] = repo.NewState
		}
		currentStates.mutex.Unlock()
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		return err
	}

	return err
}
