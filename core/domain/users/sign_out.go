package users

import (
	"context"
	"errors"
	"os"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/libs/graphql-go"
)

func SignOut() error {
	client := api.Client()
	if !client.IsAuthenticated() || client.SessionID == nil {
		return errors.New("You are not authenticated. Aborting sign out operation.")
	}

	err := DeletePersistedSession()
	if err != nil {
		return err
	}

	input := model.RevokeSessionInput{
		ID: *client.SessionID,
	}
	var resp struct {
		RevokeSession *bool `json:"revokeSession"`
	}
	req := graphql.NewRequest(`
        mutation ($input: RevokeSessionInput!) {
			revokeSession(input: $input)
		}
	`)
	req.Var("input", input)

	err = client.Do(context.Background(), req, &resp)
	if err == nil {
		client.Deauthenticate()
		os.Remove(db.DBFilePath)
	}

	return err
}
