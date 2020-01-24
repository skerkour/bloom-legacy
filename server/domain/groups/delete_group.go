package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error

	if twerr := CheckUserIsGroupAdmin(ctx, tx, user.ID, group.ID); twerr != nil {
		return twerr
	}

	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, group.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		return twirp.InternalError(ErrorDeleteGroupMsg)
	}

	return nil
}
