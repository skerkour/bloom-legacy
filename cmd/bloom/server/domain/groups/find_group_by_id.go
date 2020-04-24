package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

func FindGroupById(ctx context.Context, tx *sqlx.Tx, id uuid.UUID) (*Group, error) {
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
		logger.Error("groups.FindGroupById: finding group", rz.Err(err),
			rz.String("group.id", id.String()))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
