package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// CreateUser insert an user in the database
func (repo *UsersRepository) CreateUser(ctx context.Context, db db.Queryer, user users.User) error {
	query := `INSERT INTO users
	(id, created_at, updated_at, disabled_at, username, email, display_name, bio, first_name, last_name,
		state, is_admin, auth_key_hash, public_key, encrypted_private_key, private_key_nonce,
		encrypted_master_key, master_key_nonce)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)`
	_, err := db.Exec(ctx, query,
		user.ID, user.CreatedAt, user.UpdatedAt, user.DisabledAt, user.Username, user.Email, user.DisplayName,
		user.Bio, user.FirstName, user.LastName,
		user.State, user.IsAdmin, user.AuthKeyHash, user.PublicKey, user.EncryptedPrivateKey, user.PrivateKeyNonce,
		user.EncryptedMasterKey, user.MasterKeyNonce)

	if err == nil {
		repo.cache.Set(getUserCacheKey(user.ID), user)
	} else {
		logger := log.FromCtx(ctx)
		const errMessage = "users.CreateUser: inserting user"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", user.ID))
		err = errors.Internal(errMessage, err)
	}

	return err
}
