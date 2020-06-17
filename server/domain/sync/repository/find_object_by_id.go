package repository

/*

import (
	"context"
	"database/sql"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

// FindObjectByID finds an object by its id. returns no error but nil if object not found
func FindObjectByID(ctx context.Context, tx *sqlx.Tx, objectID []byte, forUpdate bool) (ret *Object, err error) {
	var object Object
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM objects WHERE id = $1"
	if forUpdate {
		queryFind += " FOR UPDATE"
	}
	if tx == nil {
		err = db.DB.Get(&object, queryFind, objectID)
	} else {
		err = tx.Get(&object, queryFind, objectID)
	}
	if err != nil {
		if err == sql.ErrNoRows {
			err = nil
			return
		}
		logger.Error("objects.FindObjectByID: finding object", rz.Err(err),
			rz.String("object.id", base64.StdEncoding.EncodeToString(objectID)))
		return ret, NewError(ErrorInternal)
	}

	ret = &object
	return ret, err
}
*/
