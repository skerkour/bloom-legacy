package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindGroupById(ctx context.Context, id string) (*Group, error) {
	ret := &Group{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM groups WHERE id = $1"
	err = db.DB.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("finding user", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
