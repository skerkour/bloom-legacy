package db

import (
	"context"
	"database/sql"
	"time"

	// import pgx driver
	_ "github.com/jackc/pgx/v4/stdlib"
	"gitlab.com/bloom42/gobox/database/sqlx"
)

// Queryer allows to query a database.
type Queryer interface {
	Get(ctx context.Context, dest interface{}, query string, args ...interface{}) error
	Select(ctx context.Context, dest interface{}, query string, args ...interface{}) error
	Query(ctx context.Context, query string, args ...interface{}) (*sql.Rows, error)
	Exec(ctx context.Context, query string, args ...interface{}) (sql.Result, error)
}

// DB represents a pool of zero or more underlying connections. It must be safe for concurrent use
// by multiple goroutines.
type DB interface {
	Ping(ctx context.Context) error
	SetMaxIdleConns(n int)
	SetMaxOpenConns(n int)
	SetConnMaxLifetime(d time.Duration)
	Stats() sql.DBStats
	Queryer
	Txer
}

// Txer is the ability to start transactions
type Txer interface {
	Begin(ctx context.Context) (Tx, error)
	BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error)
}

// Tx represents an in-progress database transaction.
type Tx interface {
	Commit() error
	Rollback() error
	Queryer
}

// Connect to a database and verify the connections with a ping.
func Connect(databaseURL string, poolSize int) (ret *Database, err error) {
	sqlxDB, err := sqlx.Connect("pgx", databaseURL)
	if err != nil {
		return
	}

	ret = &Database{
		sqlxDB: sqlxDB,
	}

	ret.SetMaxOpenConns(poolSize)
	return
}
