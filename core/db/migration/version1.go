package migration

import (
	"github.com/jmoiron/sqlx"
)

type Version1 struct{}

func (v Version1) Run(db *sqlx.DB, userVersion int) error {
	if userVersion >= 1 {
		return nil
	}

	tx, err := db.Beginx()
	if err != nil {
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS preferences (
		key TEXT PRIMARY KEY NOT NULL,
		value TEXT NOT NULL
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS groups (
		id TEXT PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		name TEXT NOT NULL,
		description TEXT NOT NULL,
		avatar_url TEXT,
		state string NOT NULL
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS objects (
		id BLOB PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		updated_at DATETIME NOT NULL,
		type TEXT NOT NULL,
		data JSON NOT NULL,
		out_of_sync INTEGER NOT NULL,
		group_id TEXT
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec("PRAGMA user_version = 1")
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
