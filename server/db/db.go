package db

import (
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
	"gitlab.com/bloom42/bloom/server/config"
)

var DB *sqlx.DB

func Init() error {
	var err error

	DB, err = sqlx.Connect("postgres", config.Database.URL)
	if err != nil {
		return err
	}

	DB.SetMaxOpenConns(config.Database.PoolSize)

	err = DB.Ping()
	if err != nil {
		return err
	}

	return nil
}
