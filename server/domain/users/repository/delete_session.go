package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeleteSession deletes a session in the database
func (repo *UsersRepository) DeleteSession(ctx context.Context, db db.Queryer, sessionID uuid.UUID) error {
	query := `DELETE FROM sessions WHERE id = $1`
	_, err := db.Exec(ctx, query, sessionID)

	if err == nil {
		repo.cache.Delete(getSessionCacheKey(sessionID))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.DeleteSession: deleting session"
		logger.Error(errMessage, log.Err("error", err), log.UUID("session.id", sessionID))
		err = errors.Internal(errMessage, err)
	}
	return err
}
