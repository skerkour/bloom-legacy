package users

import (
	"context"
	"encoding/json"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

// SaveSignedIn saves signedIn info in preferences
func SaveSignedIn(signin *model.SignedIn) error {
	ctx := context.Background()

	tx, err := db.DB.Beginx()
	if err != nil {
		return err
	}

	err = SaveMe(ctx, tx, signin.Me)
	if err != nil {
		tx.Rollback()
		return err
	}

	err = SaveSession(ctx, tx, signin.Session)
	if err != nil {
		tx.Rollback()
		return err
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
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

	err = preferences.Set(ctx, tx, ME_KEY, string(meData))
	return err
}

// SaveSession saves session in preferences
func SaveSession(ctx context.Context, tx *sqlx.Tx, session *model.Session) error {
	sessionData, err := json.Marshal(session)
	if err != nil {
		return err
	}

	err = preferences.Set(ctx, tx, SESSION_KEY, string(sessionData))
	return err
}
