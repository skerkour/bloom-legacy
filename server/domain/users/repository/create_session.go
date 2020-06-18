package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// CreateSession insert a session in the database
func (repo *UsersRepository) CreateSession(ctx context.Context, db db.Queryer, session users.Session) error {
	query := `INSERT INTO sessions
	(id, created_at, updated_at, user_id, hash, device_os, device_type)
	VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err := db.Exec(ctx, query, session.ID, session.CreatedAt, session.UpdatedAt, session.UserID,
		session.Hash, session.DeviceOS, session.DeviceType)

	if err == nil {
		repo.cache.Set(getSessionCacheKey(session.ID), session)
	} else {
		logger := log.FromCtx(ctx)
		errMessage := "users.CreateSession: inserting session"
		logger.Error(errMessage, log.Err("error", err), log.UUID("session.id", session.ID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
