package keys

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupMasterKey(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID) ([]byte, error) {
	var key []byte
	var err error

	query := "SELECT master_key FROM groups WHERE id = ?"
	if tx == nil {
		err = db.DB.Get(&key, query, groupID)
	} else {
		err = tx.Get(&key, query, groupID)
	}

	return key, err
}
