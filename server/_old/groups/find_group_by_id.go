package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupById(ctx context.Context, tx *sqlx.Tx, id uuid.UUID, forUpdate bool) (*Group, error) {
	ret := &Group{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM groups WHERE id = $1"
	if forUpdate {
		queryFind += " FOR UPDATE"
	}
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
