package objects

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/lily/graphql"
	"gitlab.com/bloom42/lily/uuid"
)

func push() error {
	var err error
	client := api.Client()

	input := model.PushInput{
		Repositories: []*model.RepositoryPushInput{},
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
		repo := &model.RepositoryPushInput{
			CurrentState: state,
			GroupID:      groupID,
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

	return nil
}
