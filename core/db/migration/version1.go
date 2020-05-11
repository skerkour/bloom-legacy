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
	CREATE TABLE IF NOT EXISTS notes (
		id TEXT PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		updated_at DATETIME NOT NULL,
		archived_at DATETIME,
		title TEXT NOT NULL,
		body TEXT NOT NULL,
		color TEXT NOT NULL,
		is_pinned INTEGER
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS calendar_events (
		id TEXT PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		updated_at DATETIME NOT NULL,
		title TEXT NOT NULL,
		description TEXT NOT NULL,
		start_at DATETIME NOT NULL,
		end_at DATETIME NOT NULL
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS contacts (
		id TEXT PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		updated_at DATETIME NOT NULL,
		first_name TEXT NOT NULL,
		last_name TEXT NOT NULL,
		notes TEXT NOT NULL,
		addresses TEXT NOT NULL,
		birthday TEXT,
		organizations TEXT NOT NULL,
		emails TEXT NOT NULL,
		phones TEXT NOT NULL,
		websites TEXT NOT NULL,
		device_id TEXT NOT NULL,
		bloom_username TEXT NOT NULL
	)
	`)
	if err != nil {
		tx.Rollback()
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
		avatar_url TEXT
	)
	`)
	if err != nil {
		tx.Rollback()
		return err
	}

	_, err = tx.Exec(`
	CREATE TABLE IF NOT EXISTS objects (
		id TEXT PRIMARY KEY NOT NULL,
		type TEXT PRIMARY NOT NULL,
		data JSON PRIMARY NOT NULL,
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
