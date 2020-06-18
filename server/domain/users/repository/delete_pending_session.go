package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeletePendingSession deletes a pending session in the database
func (repo *UsersRepository) DeletePendingSession(ctx context.Context, db db.Queryer, pendingSessionID uuid.UUID) error {
	query := `DELETE FROM pending_sessions WHERE id = $1`
	_, err := db.Exec(ctx, query, pendingSessionID)

	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "users.DeletePendingSession: deleting pending session"
		logger.Error(errMessage, log.Err("error", err), log.UUID("pending_session.id", pendingSessionID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
