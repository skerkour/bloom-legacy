package users

import (
	"context"
	"encoding/json"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

// SaveSignedIn saves signedIn info in preferences
func SaveSignedIn(ctx context.Context, tx *sqlx.Tx, signin *model.SignedIn) error {
	var err error

	err = SaveMe(ctx, tx, signin.Me)
	if err != nil {
		return err
	}

	err = SaveSession(ctx, tx, signin.Session)
	if err != nil {
		return err
	}
	return nil
}

// SaveMe saves me in preferences
func SaveMe(ctx context.Context, tx *sqlx.Tx, me *model.User) error {
	meData, err := json.Marshal(me)
	if err != nil {
		return err
	}

	err = preferences.Set(ctx, tx, PREFERENCES_KEY_ME, string(meData))
	return err
}

// SaveSession saves session in preferences
func SaveSession(ctx context.Context, tx *sqlx.Tx, session *model.Session) error {
	sessionData, err := json.Marshal(session)
	if err != nil {
		return err
	}

	err = preferences.Set(ctx, tx, PREFERENCES_KEY_SESSION, string(sessionData))
	return err
}
