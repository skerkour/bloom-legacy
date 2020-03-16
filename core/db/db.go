package db

import (
	"os"
	"path/filepath"

	"github.com/jmoiron/sqlx"
	// import sqlite drivers
	_ "github.com/mattn/go-sqlite3"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

// DB is a pointer to the singleton DB instance
var DB *sqlx.DB

// DBFilePath is the path of the app's database
var DBFilePath string

func dbPath(directory string) (string, error) {
	return filepath.Join(directory, "bloom.db"), nil
}

// Init initializes the DB singleton and make migrations if necessary
func Init() error {
	dbDir, err := kernel.AppDirectory()
	if err != nil {
		return err
	}

	err = os.MkdirAll(dbDir, 0740)
	if err != nil {
		return err
	}

	DBFilePath, err = dbPath(dbDir)
	if err != nil {
		return err
	}

	DB, err = sqlx.Open("sqlite3", DBFilePath)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
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
		return err
	}

	_, err = DB.Exec(`
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
		return err
	}

	_, err = DB.Exec(`
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
		device_id TEXT NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS preferences (
		key TEXT PRIMARY KEY NOT NULL,
		value TEXT NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS groups (
		id TEXT PRIMARY KEY NOT NULL,
		created_at DATETIME NOT NULL,
		name TEXT NOT NULL,
		description TEXT NOT NULL,
		avatar_url TEXT
	)
	`)
	if err != nil {
		return err
	}

	return err
}
