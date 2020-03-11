package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindAllGroups(ctx context.Context) ([]Group, error) {
	ret := []Group{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM groups"
	err = db.DB.Select(&ret, queryFind)
	if err != nil {
		logger.Error("groups.FindAllGroups: finding groups", rz.Err(err))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
