package objects

import (
	"context"
	"encoding/base64"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/groups"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/graphql"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
)

func pull(init bool) error {
	var err error
	client := api.Client()
	ctx := context.Background()
	var masterKey []byte
	sinceState := "" // on initial sync, we request an empty state

	// prepare api request
	input := model.PullInput{
		Repositories: []*model.RepositoryPullInput{},
	}

	myGroups, err := groups.FindGroups(ctx, nil)
	if err != nil {
		return err
	}

	for _, group := range myGroups.Groups {
		repo := &model.RepositoryPullInput{
			SinceState: group.State,
			GroupID:    &group.ID,
		}
		input.Repositories = append(input.Repositories, repo)
	}
	if init != true {
		sinceState = *kernel.Me.State
	}
	myRepo := &model.RepositoryPullInput{
		SinceState: sinceState,
		GroupID:    nil,
	}
	input.Repositories = append(input.Repositories, myRepo)

	// build api request
	var resp struct {
		Pull *model.Pull `json:"pull"`
	}
	req := graphql.NewRequest(`
		query ($input: PullInput!) {
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
		return err
	}

	log.Debug("Received from pull", rz.Any("resp", resp))

	tx, err := db.DB.Beginx()
	if err != nil {
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
			log.Debug("decrypting object", rz.String("objects.id", base64.StdEncoding.EncodeToString(object.ID)))
			decryptedObject, err := decryptObject(object, masterKey)
			if err != nil {
				log.Debug("Error decrypting object", rz.Err(err))
				tx.Rollback()
				return err
			}

			// check if object exist and is out of sync
			ofsStoredObject, err := FindOutOfSyncObjectByID(ctx, tx, decryptedObject.ID)
			if ofsStoredObject != nil {
				log.Debug("CONFLICT FOUND WHILE PULLING. Starting to dedup object", rz.Any("OFS object", ofsStoredObject))
				// resolve conflict
				// create a new object from the local out of sync object (with a new id)
				dedupedObject, err := dedupObject(ofsStoredObject, []byte(kernel.Me.Username))
				if err != nil {
					log.Debug("Error deduping object", rz.Err(err))
					tx.Rollback()
					return err
				}
				// empty object
				if len(decryptedObject.Data) < 3 {
					// remove local object
					err = DeleteObject(ctx, tx, decryptedObject.ID)
					if err != nil {
						tx.Rollback()
						return err
					}
				} else {
					err = SaveObject(ctx, tx, decryptedObject)
					if err != nil {
						tx.Rollback()
						return err
					}
				}
				// in all situations, save deduped (local OFS) object
				err = SaveObject(ctx, tx, dedupedObject)
				if err != nil {
					tx.Rollback()
					return err
				}

			} else {
				if len(decryptedObject.Data) < 3 {
					// remove local object
					err = DeleteObject(ctx, tx, object.ID)
					if err != nil {
						tx.Rollback()
						return err
					}
				} else {
					err = SaveObject(ctx, tx, decryptedObject)
					if err != nil {
						tx.Rollback()
						return err
					}
				}
			}

		}
	}

	// update groups and me states
	for _, repo := range resp.Pull.Repositories {
		if repo.GroupID != nil {
			err = groups.SaveGroupState(ctx, tx, *(repo.GroupID), repo.NewState)
		} else {
			kernel.Me.State = &repo.NewState
			err = users.SaveMe(ctx, tx, kernel.Me)
		}
		if err != nil {
			log.Debug("Error saving state")
			tx.Rollback()
			return err
		}
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		return err
	}

	return err
}
