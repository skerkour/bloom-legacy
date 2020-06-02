package users

import (
	"context"

	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func DeletePersistedSession() error {
	ctx := context.Background()

	tx, err := db.DB.Beginx()
	if err != nil {
		return err
	}

	err = preferences.Delete(ctx, tx, "me")
	if err != nil {
		tx.Rollback()
		return err
	}

	err = preferences.Delete(ctx, tx, "session")
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
