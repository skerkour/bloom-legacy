package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeletePendingUser deletes a pending user in the database
func (repo *UsersRepository) DeletePendingUser(ctx context.Context, db db.Queryer, pendingUserID uuid.UUID) error {
	query := `DELETE FROM pending_users WHERE id = $1`
	_, err := db.Exec(ctx, query, pendingUserID)

	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "users.DeletePendingUser: deleting pending user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("pending_user.id", pendingUserID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
