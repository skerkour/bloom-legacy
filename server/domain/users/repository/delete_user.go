package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// DeleteUser deletes an user in the database
func (repo *UsersRepository) DeleteUser(ctx context.Context, db db.Queryer, userID uuid.UUID) error {
	query := `DELETE FROM users WHERE id = $1`
	_, err := db.Exec(ctx, query, userID)

	if err == nil {
		repo.cache.Delete(getUserCacheKey(userID))
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.DeleteUser: deleting user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
