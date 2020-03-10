package db

import (
	"bytes"
	"database/sql"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"

	_ "github.com/mattn/go-sqlite3"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

var DB *sql.DB

func homeDir() (string, error) {
	if runtime.GOOS == "android" {
		data, err := ioutil.ReadFile("/proc/self/cmdline")
		if err != nil {
			return "", err
		}
		return filepath.Join("data", "data", string(bytes.Trim(data, "\x00"))), nil
	} else {
		home, err := kernel.GetHomeDirectory()
		if err != nil {
			return "", err
		}
		return filepath.Join(home, ".bloom"), nil
	}
}

func dbDir() (string, error) {
	return homeDir()
	//return filepath.Join(home, "db"), err
}

func dbPath() (string, error) {
	dbDir, err := dbDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(dbDir, "bloom.db"), nil

}

func Init() error {
	dbDir, err := dbDir()
	if err != nil {
		return err
	}

	err = os.MkdirAll(dbDir, 0740)
	if err != nil {
		return err
	}

	dbPath, err := dbPath()
	if err != nil {
		return err
	}

	DB, err = sql.Open("sqlite3", dbPath)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS notes (
		id TEXT PRIMARY KEY NOT NULL,
		createdAt DATETIME NOT NULL,
		updatedAt DATETIME NOT NULL,
		archivedAt DATETIME,
		title TEXT NOT NULL,
		body TEXT NOT NULL,
		color TEXT NOT NULL,
		isPinned INTEGER
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS calendar_events (
		id TEXT PRIMARY KEY NOT NULL,
		createdAt DATETIME NOT NULL,
		updatedAt DATETIME NOT NULL,
		title TEXT NOT NULL,
		description TEXT NOT NULL,
		startAt DATETIME NOT NULL,
		endAt DATETIME NOT NULL
	)
	`)
	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE IF NOT EXISTS contacts (
		id TEXT PRIMARY KEY NOT NULL,
		createdAt DATETIME NOT NULL,
		updatedAt DATETIME NOT NULL,
		firstName TEXT NOT NULL,
		lastName TEXT NOT NULL,
		notes TEXT NOT NULL,
		addresses TEXT NOT NULL,
		birthday TEXT,
		organizations TEXT NOT NULL,
		emails TEXT NOT NULL,
		phones TEXT NOT NULL,
		websites TEXT NOT NULL,
		deviceId TEXT NOT NULL
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

	return err
}
