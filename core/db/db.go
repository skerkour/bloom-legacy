package db

import (
	"os"
	"path/filepath"

	"github.com/jmoiron/sqlx"
	// import sqlite drivers
	_ "github.com/mattn/go-sqlite3"
	"gitlab.com/bloom42/bloom/core/db/migration"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

// DB is a pointer to the singleton DB instance
var DB *sqlx.DB

// FilePath is the path of the app's database
var FilePath string

func dbPath(directory string) (string, error) {
	return filepath.Join(directory, "bloom.db"), nil
}

func CloseAndRemove() error {
	if DB != nil {
		DB.Close()
	}
	os.Remove(FilePath)
	return nil
}

// Init initializes the DB singleton and make migrations if necessary
func Init(key string) error {
	var userVersion int

	dbDir, err := kernel.AppDataDir()
	if err != nil {
		return err
	}

	err = os.MkdirAll(dbDir, 0740)
	if err != nil {
		return err
	}

	FilePath, err = dbPath(dbDir)
	if err != nil {
		return err
	}

	DB, err = sqlx.Open("sqlite3", FilePath)
	if err != nil {
		return err
	}

	err = DB.Get(&userVersion, "PRAGMA user_version;")
	if err != nil {
		return err
	}

	// see https://github.com/signalapp/Signal-Desktop/blob/master/app/sql.js
	// for reference
	migrations := []migration.Version{
		migration.Version1{},
	}

	for _, migrat := range migrations {
		migrat.Run(DB, userVersion)
		userVersion += 1
	}

	return err
}
