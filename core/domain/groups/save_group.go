package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/bloom/core/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func SaveGroup(ctx context.Context, tx *sqlx.Tx, group *model.Group, groupMasterKey []byte, state string) (err error) {
	query := `INSERT INTO groups (id, created_at, name, description, avatar_url, master_key, state)
		VALUES (?, ?, ?, ?, ?, ?, ?)`
	args := []interface{}{group.ID, group.CreatedAt, group.Name, group.Description,
		group.AvatarURL, groupMasterKey, state}

	if tx == nil {
		_, err = db.DB.Exec(query, args...)
	} else {
		_, err = tx.Exec(query, args...)
	}
	return
}

func SaveGroupState(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID, newState string) (err error) {
	query := "UPDATE groups SET state = ? WHERE id = ?"
	if tx == nil {
		_, err = db.DB.Exec(query, newState, groupID)
	} else {
		_, err = tx.Exec(query, newState, groupID)
	}
	return
}
