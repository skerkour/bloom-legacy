package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func SaveGroupState(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID, newState string) (err error) {
	query := "UPDATE groups SET state = ? WHERE id = ?"
	if tx == nil {
		_, err = db.DB.Exec(query, newState, groupID)
	} else {
		_, err = tx.Exec(query, newState, groupID)
	}
	return
}
