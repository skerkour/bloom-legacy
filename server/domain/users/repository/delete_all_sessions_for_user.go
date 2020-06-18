package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *UsersRepository) DeleteAllSessionsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (err error) {
	sessions, err := repo.FindAllSessionsForUser(ctx, db, userID)
	if err != nil {
		return
	}

	query := "DELETE FROM sessions WHERE user_id = $1"

	_, err = db.Exec(ctx, query, userID)
	if err == nil {
		for _, session := range sessions {
			repo.cache.Delete(getSessionCacheKey(session.ID))
		}
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.DeleteAllSessionsForUser: deleting sessions"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return err
}
