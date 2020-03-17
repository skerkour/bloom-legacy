package migration

import (
	"github.com/jmoiron/sqlx"
)

type Version interface {
	Run(db *sqlx.DB, userVersion int) error
}
