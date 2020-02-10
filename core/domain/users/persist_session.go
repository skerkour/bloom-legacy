package users

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
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

	tx, err := db.DB.Begin()
	if err != nil {
		return err
	}

	_, err = tx.Exec("INSERT INTO preferences (key, value) VALUES (?, ?)", "me", string(meData))
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec("INSERT INTO preferences (key, value) VALUES (?, ?)", "session", string(sessionData))
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
