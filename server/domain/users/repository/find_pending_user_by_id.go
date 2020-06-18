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

// FindPendingUserByID find a pending user with the given ID. returns an error if not found
func (repo *UsersRepository) FindPendingUserByID(ctx context.Context, db db.Queryer, pendingUserID uuid.UUID) (ret users.PendingUser, err error) {
	query := `SELECT * FROM pending_users WHERE id = $1`
	err = db.Get(ctx, &ret, query, pendingUserID)

	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Pending user not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "users.FindPendingUserByID: finding pending user"
			logger.Error(errMessage, log.Err("error", err), log.UUID("pending_user.id", pendingUserID))
			err = errors.Internal(errMessage, err)
		}
	}

	return
}
