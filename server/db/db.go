package db

import (
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
	"gitlab.com/bloom42/bloom/server/config"
)

var DB *sqlx.DB

func Init(conf config.Configuration) error {
	var err error

	DB, err = sqlx.Connect("postgres", conf.Database.URL)
	if err != nil {
		return err
	}

	err = DB.Ping()
	if err != nil {
		return err
	}

	return nil
}
