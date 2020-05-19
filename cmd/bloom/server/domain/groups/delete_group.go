package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeleteGroup deletes a group. Admin role is required
func DeleteGroup(ctx context.Context, actor *users.User, groupID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.DeleteGroup: Starting transaction", rz.Err(err))
		err = NewError(ErrorDeletingGroup)
		return
	}

	if err = CheckUserIsGroupAdmin(ctx, tx, actor.ID, groupID); err != nil {
		return err
	}

	group, err := FindGroupById(ctx, tx, groupID, false)
	if err != nil {
		tx.Rollback()
		err = NewError(ErrorGroupNotFound)
		return
	}

	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, group.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		err = NewError(ErrorDeletingGroup)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.DeleteGroup: Committing transaction", rz.Err(err))
		err = NewError(ErrorDeletingGroup)
		return
	}

	return
}
