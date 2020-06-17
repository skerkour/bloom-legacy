package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

// UpdateObject update an existing object in the database
func (repo *SyncRepository) UpdateObject(ctx context.Context, db db.Queryer, object sync.Object) error {
	query := `UPDATE objects
	SET algorithm = $1, nonce = $2, encrypted_key = $3, encrypted_data = $4, updated_at_state = $5
	WHERE id = $6`

	_, err := db.Exec(ctx, query, object.Algorithm, object.Nonce, object.EncryptedKey,
		object.EncryptedData, object.UpdatedAtState, object.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "sync.UpdateObject: updating object"
		logger.Error(errMessage, log.Err("error", err), log.Base64("object.id", object.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
