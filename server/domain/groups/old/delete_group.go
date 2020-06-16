package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeleteGroup deletes a group. Admin role is required
func DeleteGroup(ctx context.Context, tx *sqlx.Tx, actor *users.User, groupID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)

	if err = CheckUserIsGroupAdmin(ctx, tx, actor.ID, groupID); err != nil {
		return err
	}

	group, err := FindGroupById(ctx, tx, groupID, false)
	if err != nil {
		err = NewError(ErrorGroupNotFound)
		return
	}

	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, group.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		err = NewError(ErrorDeletingGroup)
		return
	}

	return
}
