package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/groups"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/users"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/graphql"
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

	myGroups, err := groups.FindGroups(ctx, tx)
	if err != nil {
		tx.Rollback()
		return err
	}

	for _, group := range myGroups.Groups {
		repo := &model.RepositoryPullInput{
			SinceState: group.State,
			GroupID:    &group.ID,
		}
		input.Repositories = append(input.Repositories, repo)
	}
	myRepo := &model.RepositoryPullInput{
		SinceState: *kernel.Me.State,
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
			if decryptedObject == nil {
				// remove local object
				err = DeleteObject(ctx, tx, object.ID)
				if err != nil {
					tx.Rollback()
					return err
				}
			}
			// check if object exist and is out of sync
			ofsStoredObject, err := FindOutOfSyncObjectByID(ctx, tx, decryptedObject.ID)
			if ofsStoredObject != nil {
				// resolve conflict
				// create a new object from the local out of sync object (with a new id)
				dedupedObject, err := dedupObject(ofsStoredObject, []byte(kernel.Me.Username))
				if err != nil {
					tx.Rollback()
					return err
				}
				err = SaveObject(ctx, tx, dedupedObject)
				if err != nil {
					tx.Rollback()
					return err
				}
			} else {
				// save object
				err = SaveObject(ctx, tx, decryptedObject)
				if err != nil {
					tx.Rollback()
					return err
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
