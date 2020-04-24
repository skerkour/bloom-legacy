package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
)

func DeleteGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) error {
	logger := rz.FromCtx(ctx)
	var err error

	if err = CheckUserIsGroupAdmin(ctx, tx, user.ID.String(), group.ID); err != nil {
		return err
	}

	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, group.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		return NewError(ErrorDeletingGroup)
	}

	return nil
}
