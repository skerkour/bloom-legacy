package db

import (
	"bytes"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"

	"github.com/jmoiron/sqlx"
	_ "github.com/mattn/go-sqlite3"
	"gitlab.com/bloom42/bloom/core/domain/kernel"
)

var DB *sqlx.DB
var DBFilePath string

func appDirectory() (string, error) {
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
	return appDirectory()
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

	DBFilePath, err = dbPath()
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
