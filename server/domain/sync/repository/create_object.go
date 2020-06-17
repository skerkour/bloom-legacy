package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// CreateObject insert an object in the database
func (repo *SyncRepository) CreateObject(ctx context.Context, db db.Queryer, object sync.Object) error {
	query := `INSERT INTO objects
	(id, updated_at_state, algorithm, nonce, encrypted_key, encrypted_data, group_id, user_id)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	_, err := db.Exec(query, object.ID, object.UpdatedAtState, object.Algorithm,
		object.Nonce, object.EncryptedKey, object.EncryptedData, object.GroupID, object.UserID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "sync.CreateObject: inserting object"
		logger.Error(errMessage, log.Err("error", err), log.Base64("object.id", object.ID))
		err = errors.Internal(errMessage, err)
	}

	return
}
