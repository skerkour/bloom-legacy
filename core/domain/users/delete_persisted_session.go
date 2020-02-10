package users

import (
	"gitlab.com/bloom42/bloom/core/db"
)

func DeletePersistedSession() error {
	tx, err := db.DB.Begin()
	if err != nil {
		return err
	}

	_, err = tx.Exec("DELETE FROM  preferences WHERE key = ?", "me")
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec("DELETE FROM  preferences WHERE key = ?", "session")
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
