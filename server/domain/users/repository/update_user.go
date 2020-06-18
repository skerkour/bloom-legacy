package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// UpdateUser update an user in the database
func (repo *UsersRepository) UpdateUser(ctx context.Context, db db.Queryer, user users.User) error {
	query := `UPDATE users SET
			updated_at = $1, disabled_at = $2, avatar = $3, email = $4, display_name = $5, auth_key_hash = $6,
			bio = $7, first_name = $8, last_name = $9, is_admin = $10, public_key = $11, private_key_nonce = $12,
			encrypted_master_key = $13, master_key_nonce = $14, state = $15
		WHERE id = $16`
	_, err := db.Exec(ctx, query, user.UpdatedAt, user.DisabledAt, user.Avatar, user.Email, user.DisplayName,
		user.AuthKeyHash, user.Bio, user.FirstName, user.LastName, user.IsAdmin, user.PublicKey, user.PrivateKeyNonce,
		user.EncryptedMasterKey, user.MasterKeyNonce, user.State, user.ID)

	if err == nil {
		repo.cache.Set(getUserCacheKey(user.ID), user)
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.UpdateUser: updating user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", user.ID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
