package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
	"gitlab.com/bloom42/lily/uuid"
)

func pull() error {
	var err error
	client := api.Client()

	input := model.PullInput{
		Repositories: []*model.RepositoryPullInput{},
	}

	currentStates.mutex.RLock()
	for groupIDStr, state := range currentStates.states {
		var groupID *uuid.UUID

		if groupIDStr != "" {
			groupUUID, err2 := uuid.Parse(groupIDStr)
			if err2 != nil {
				return err2
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

	err = client.Do(context.Background(), req, &resp)
	if err != nil {
		return err
	}

	currentStates.mutex.Lock()
	for _, repo := range resp.Pull.Repositories {
		if repo.GroupID != nil {
			currentStates.states[repo.GroupID.String()] = repo.NewState
		} else {
			currentStates.states[""] = repo.NewState
		}
	}
	currentStates.mutex.Unlock()

	// TODO: resolve conflicts

	return err
}
