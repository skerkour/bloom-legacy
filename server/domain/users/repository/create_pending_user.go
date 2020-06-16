package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// CreatePendingUser insert a pending user in the database
func (repo *UsersRepository) CreatePendingUser(ctx context.Context, db db.Queryer, pendingUser users.PendingUser) error {
	query := `INSERT INTO pending_users
		(id, created_at, updated_at, email, display_name, code_hash, failed_attempts, verified_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err = db.Exec(query, pendingUser.ID, pendingUser.CreatedAt, pendingUser.UpdatedAt, pendingUser.Email,
		pendingUser.DisplayName, pendingUser.CodeHash, pendingUser.FailedAttempts, pendingUser.VerifiedAt)
	if err != nil {
		logger := log.FromCtx(ctx)
		errMessage := "users.CreatePendingUser: inserting pending user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("pending_user.id", pendingUser.ID))
		err = errors.Internal("users.CreatePendingUser: inserting pending user", err)
	}

	return err
}
