package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

func deleteSession(ctx context.Context, tx *sqlx.Tx, sessionId, userId uuid.UUID) error {
	logger := rz.FromCtx(ctx)

	queryDeleteSession := "DELETE FROM sessions WHERE id = $1 AND user_id = $2"
	_, err := tx.Exec(queryDeleteSession, sessionId, userId)
	if err != nil {
		logger.Error("users.DeleteSession: error deleting sessiong", rz.Err(err),
			rz.String("session.id", sessionId.String()), rz.String("user.id", userId.String()))
		return NewError(ErrorDeletingSession)
	}
	return nil
}
