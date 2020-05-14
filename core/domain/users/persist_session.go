package users

import (
	"context"
	"encoding/json"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func PersistSession(signin *model.SignedIn) error {
	meData, err := json.Marshal(signin.Me)
	if err != nil {
		return err
	}

	sessionData, err := json.Marshal(signin.Session)
	if err != nil {
		return err
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		return err
	}

	err = preferences.Set(context.Background(), tx, "me", string(meData))
	if err != nil {
		tx.Rollback()
		return err
	}

	err = preferences.Set(context.Background(), tx, "session", string(sessionData))
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

func SaveMe(ctx context.Context, tx *sqlx.Tx, me *model.User) error {
	meData, err := json.Marshal(me)
	if err != nil {
		return err
	}

	err = preferences.Set(context.Background(), tx, "me", string(meData))
	return err
}
