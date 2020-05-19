package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func deletePendingUser(ctx context.Context, tx *sqlx.Tx, pendingUserID uuid.UUID) error {
	logger := rz.FromCtx(ctx)

	queryDeletePendingUser := "DELETE FROM pending_users WHERE id = $1"
	_, err := tx.Exec(queryDeletePendingUser, pendingUserID)
	if err != nil {
		logger.Error("users.deletePendingUser: error deleting pending user", rz.Err(err), rz.String("pending_user.id", pendingUserID.String()))
		return NewError(ErrorCompletingRegistration)
	}
	return nil
}
