package db

import (
	"context"
	"database/sql"
	"time"

	"gitlab.com/bloom42/gobox/database/sqlx"
)

// Database is wrapper of `sqlx.DB` which implements `DB`
type Database struct {
	sqlxDB *sqlx.DB
}

// Ping verifies a connection to the database is still alive, establishing a connection if necessary.
func (db *Database) Ping(ctx context.Context) error {
	return db.sqlxDB.PingContext(ctx)
}

// SetConnMaxLifetime sets the maximum amount of time a connection may be reused.
func (db *Database) SetConnMaxLifetime(d time.Duration) {
	db.sqlxDB.SetConnMaxLifetime(d)
}

// SetMaxIdleConns sets the maximum number of connections in the idle connection pool.
func (db *Database) SetMaxIdleConns(n int) {
	db.sqlxDB.SetMaxIdleConns(n)
}

// SetMaxOpenConns sets the maximum number of open connections to the database.
func (db *Database) SetMaxOpenConns(n int) {
	db.sqlxDB.SetMaxOpenConns(n)
}

// Stats returns database statistics.
func (db *Database) Stats() sql.DBStats {
	return db.sqlxDB.Stats()
}

// Begin starts a transaction. The default isolation level is dependent on the driver.
// The provided context is used until the transaction is committed or rolled back. If the context is
// canceled, the sql package will roll back the transaction. Tx.Commit will return an error if the
// context provided to BeginTx is canceled.
//
func (db *Database) Begin(ctx context.Context) (Tx, error) {
	sqlxTx, err := db.sqlxDB.BeginTxx(ctx, nil)
	return &Transaction{sqlxTx}, err
}

// BeginTx starts a transaction.
//
// The provided context is used until the transaction is committed or rolled back. If the context is
// canceled, the sql package will roll back the transaction. Tx.Commit will return an error if the
// context provided to BeginTx is canceled.
//
// The provided TxOptions is optional and may be nil if defaults should be used. If a non-default
// isolation level is used that the driver doesn't support, an error will be returned.
func (db *Database) BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error) {
	sqlxTx, err := db.sqlxDB.BeginTxx(ctx, opts)
	return &Transaction{sqlxTx}, err
}

// Exec executes a query without returning any rows. The args are for any placeholder parameters in the query.
func (db *Database) Exec(ctx context.Context, query string, args ...interface{}) (sql.Result, error) {
	return db.sqlxDB.ExecContext(ctx, query, args...)
}

// Get a single record. Any placeholder parameters are replaced with supplied args. An `ErrNoRows`
// error is returned if the result set is empty.
func (db *Database) Get(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return db.sqlxDB.GetContext(ctx, dest, query, args...)
}

// Query executes a query that returns rows, typically a SELECT. The args are for any placeholder
// parameters in the query.
func (db *Database) Query(ctx context.Context, query string, args ...interface{}) (*sql.Rows, error) {
	return db.sqlxDB.QueryContext(ctx, query, args...)
}

// Select an array of records. Any placeholder parameters are replaced with supplied args.
func (db *Database) Select(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return db.sqlxDB.SelectContext(ctx, dest, query, args...)
}

// Transaction is wrapper of `sqlx.Tx` which implements `Tx`
type Transaction struct {
	sqlxTx *sqlx.Tx
}

// Commit commits the transaction.
func (tx *Transaction) Commit() error {
	return tx.sqlxTx.Commit()
}

// Rollback aborts the transaction.
func (tx *Transaction) Rollback() error {
	return tx.sqlxTx.Rollback()
}

// Exec executes a query without returning any rows. The args are for any placeholder parameters in the query.
func (tx *Transaction) Exec(ctx context.Context, query string, args ...interface{}) (sql.Result, error) {
	return tx.sqlxTx.ExecContext(ctx, query, args...)
}

// Get a single record. Any placeholder parameters are replaced with supplied args. An `ErrNoRows`
// error is returned if the result set is empty.
func (tx *Transaction) Get(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return tx.sqlxTx.GetContext(ctx, dest, query, args...)
}

// Query executes a query that returns rows, typically a SELECT. The args are for any placeholder
// parameters in the query.
func (tx *Transaction) Query(ctx context.Context, query string, args ...interface{}) (*sql.Rows, error) {
	return tx.sqlxTx.QueryContext(ctx, query, args...)
}

// Select an array of records. Any placeholder parameters are replaced with supplied args.
func (tx *Transaction) Select(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return tx.sqlxTx.SelectContext(ctx, dest, query, args...)
}
