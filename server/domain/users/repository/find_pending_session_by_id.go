package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// FindPendingSessionByID find a session user with the given ID. returns an error if not found
func (repo *UsersRepository) FindPendingSessionByID(ctx context.Context, db db.Queryer, pendingSessionID uuid.UUID) (ret users.PendingSession, err error) {
	query := "SELECT * FROM pending_sessions WHERE id = $1"

	err = db.Get(ctx, &ret, query, pendingSessionID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Pending session not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "users.FindPendingSessionByID: finding pending session"
			logger.Error(errMessage, log.Err("error", err), log.UUID("pending_session.id", pendingSessionID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
