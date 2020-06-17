package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func deleteSession(ctx context.Context, tx *sqlx.Tx, sessionID, userID uuid.UUID) error {
	logger := rz.FromCtx(ctx)

	queryDeleteSession := "DELETE FROM sessions WHERE id = $1 AND user_id = $2"
	_, err := tx.Exec(queryDeleteSession, sessionID, userID)
	if err != nil {
		logger.Error("users.DeleteSession: error deleting sessiong", rz.Err(err),
			rz.String("session.id", sessionID.String()), rz.String("user.id", userID.String()))
		return NewError(ErrorDeletingSession)
	}
	return nil
}
