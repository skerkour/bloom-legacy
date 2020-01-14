package db

import (
	"database/sql"
	_ "github.com/mattn/go-sqlite3"
)

var DB *sql.DB

func Init() error {
	var err error

	DB, err = sql.Open("sqlite3", "/Users/user/.bloom/db/bloom.db")
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS notes (
		id TEXT PRIMARY KEY NOT NULL,
		created_at TEXT NOT NULL,
		updated_at TEXT NOT NULL,
		archived_at TEXT,
		title TEXT NOT NULL,
		body TEXT NOT NULL,
		color TEXT NOT NULL,
		is_pinned INTEGER
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS calendar_events (
		id TEXT PRIMARY KEY NOT NULL,
		created_at TEXT NOT NULL,
		updated_at TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT NOT NULL,
		start_at TEXT NOT NULL,
		end_at TEXT NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS contacts (
		id TEXT PRIMARY KEY NOT NULL,
		created_at TEXT NOT NULL,
		updated_at TEXT NOT NULL,
		first_name TEXT NOT NULL,
		last_name TEXT NOT NULL,
		notes TEXT NOT NULL,
		addresses TEXT NOT NULL,
		birthday TEXT,
		organizations TEXT NOT NULL,
		emails TEXT NOT NULL,
		phones TEXT NOT NULL,
		websites TEXT NOT NULL,
		device_id TEXT NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS contacts (
		id TEXT PRIMARY KEY NOT NULL,
		created_at TEXT NOT NULL,
		updated_at TEXT NOT NULL,
		first_name TEXT NOT NULL,
		last_name TEXT NOT NULL,
		notes TEXT NOT NULL,
		addresses TEXT NOT NULL,
		birthday TEXT,
		organizations TEXT NOT NULL,
		emails TEXT NOT NULL,
		phones TEXT NOT NULL,
		websites TEXT NOT NULL,
		device_id TEXT NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	return err
}
