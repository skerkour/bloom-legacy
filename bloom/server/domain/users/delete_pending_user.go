package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeletePendingUser(ctx context.Context, tx *sqlx.Tx, pendingUserId string) error {
	logger := rz.FromCtx(ctx)

	queryDeletePendingUser := "DELETE FROM pending_users WHERE id = $1"
	_, err := tx.Exec(queryDeletePendingUser, pendingUserId)
	if err != nil {
		logger.Error("error deleting pending user", rz.Err(err), rz.String("pending_user_id", pendingUserId))
		return NewError(ErrorCompletingRegistration)
	}
	return nil
}
