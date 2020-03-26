package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteSession(ctx context.Context, tx *sqlx.Tx, sessionId, userId string) error {
	logger := rz.FromCtx(ctx)

	queryDeleteSession := "DELETE FROM sessions WHERE id = $1 AND user_id = $2"
	_, err := tx.Exec(queryDeleteSession, sessionId, userId)
	if err != nil {
		logger.Error("users.DeleteSession: error deleting sessiong", rz.Err(err),
			rz.String("session_id", sessionId), rz.String("user_id", userId))
		return NewError(ErrorDeletingSession)
	}
	return nil
}
