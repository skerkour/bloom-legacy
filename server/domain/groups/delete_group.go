package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteGroup(ctx context.Context, tx *sqlx.Tx, groupId string) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error
	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, groupId)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		return twirp.InternalError(ErrorDeleteGroupMsg)
	}

	// delete members
	queryDeleteGroupMembers := "DELETE FROM groups_members WHERE group_id = $1"
	_, err = tx.Exec(queryDeleteGroupMembers, groupId)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleteing members", rz.Err(err))
		return twirp.InternalError(ErrorDeleteGroupMsg)
	}

	return nil
}
