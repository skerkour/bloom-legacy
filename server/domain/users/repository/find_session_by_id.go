package repository

import (
	"context"
	"database/sql"
	"fmt"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// FindSessionByID find a session with the given ID. returns an error if nut found
func (repo *UsersRepository) FindSessionByID(ctx context.Context, db db.Queryer, sessionID uuid.UUID) (ret users.Session, err error) {

	if cacheValue, found := repo.cache.Get(getSessionCacheKey(sessionID)); found {
		ret = cacheValue.(users.Session)
		return
	}

	query := `SELECT * FROM sessions WHERE id = $1`
	err = db.Get(ctx, &ret, query, sessionID)

	if err == nil {
		repo.cache.Set(getSessionCacheKey(ret.ID), ret)
	} else if err == sql.ErrNoRows {
		err = errors.NotFound(fmt.Sprintf("Session with id: %s not found", sessionID.String()))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.FindSessionByID: finding session"
		logger.Error(errMessage, log.Err("error", err), log.UUID("session.id", sessionID))
		err = errors.Internal(errMessage, err)
	}

	return
}
