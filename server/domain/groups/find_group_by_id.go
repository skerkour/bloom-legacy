package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindGroupById(ctx context.Context, tx *sqlx.Tx, id string) (*Group, error) {
	ret := &Group{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM groups WHERE id = $1"
	if tx == nil {
		err = db.DB.Get(ret, queryFind, id)
	} else {
		err = tx.Get(ret, queryFind, id)
	}
	if err != nil {
		logger.Error("finding user", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
