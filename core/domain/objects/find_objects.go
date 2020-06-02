package objects

import (
	"context"
	"database/sql"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/rz/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindOutOfSyncObjectByID(ctx context.Context, tx *sqlx.Tx, id []byte) (*Object, error) {
	ret := &Object{}
	var err error

	query := "SELECT * FROM objects WHERE out_of_sync = ? AND id = ?"
	if tx == nil {
		err = db.DB.Get(&ret, query, true, id)
	} else {
		err = tx.Get(&ret, query, true, id)
	}
	if err == sql.ErrNoRows {
		return nil, nil
	}
	// no object found
	if len(ret.ID) == 0 {
		return nil, nil
	}

	return ret, err
}

func FindOutOfSyncObjects(ctx context.Context, tx *sqlx.Tx) ([]Object, error) {
	ret := []Object{}
	var err error

	query := "SELECT * FROM objects WHERE out_of_sync = ?"
	if tx == nil {
		err = db.DB.Select(&ret, query, true)
	} else {
		err = tx.Select(&ret, query, true)
	}
	return ret, err
}

func FindObjectsByType(ctx context.Context, tx *sqlx.Tx, typ string, groupID *uuid.UUID) ([]Object, error) {
	ret := []Object{}
	var err error

	log.Debug("Finding objects by type", rz.String("type", typ), rz.Any("group.id", groupID))

	query := "SELECT * FROM objects WHERE type = ? AND length(data) > 2"
	args := []interface{}{typ}
	if groupID == nil {
		query += " AND group_id IS NULL"
	} else {
		query += " AND group_id = ?"
		args = append(args, groupID.String())
	}
	if tx == nil {
		err = db.DB.Select(&ret, query, args...)
	} else {
		err = tx.Select(&ret, query, args...)
	}

	log.Debug("Found objects", rz.Int("len(objects)", len(ret)))
	return ret, err
}
