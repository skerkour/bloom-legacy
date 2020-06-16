package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// UpdatePendingSession update a pending session in the database
func (repo *UsersRepository) UpdatePendingSession(ctx context.Context, db db.Queryer, pendingSession users.PendingSession) error {
	query := `UPDATE pending_sessions SET
		updated_at = $1, failed_attempts = $2, verified_at = $3
		WHERE id = $4`
	_, err := db.Exec(ctx, query, pendingSession.UpdatedAt, pendingSession.FailedAttempts,
		pendingSession.VerifiedAt, pendingSession.ID)

	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "users.UpdatePendingSession: updating pending session"
		logger.Error(errMessage, log.Err("error", err), log.UUID("pending_session.id", pendingSession.ID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
