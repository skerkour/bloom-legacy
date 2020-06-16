package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

// UpdateGroupState update the group's state
func UpdateGroupState(ctx context.Context, tx *sqlx.Tx, group *Group, newState int64) error {
	var err error
	logger := rz.FromCtx(ctx)

	if group.State > newState {
		return NewError(ErrorInvalidState)
	}

	group.State = newState
	group.UpdatedAt = time.Now().UTC()

	query := "UPDATE groups SET state = $1, updated_at = $2 WHERE id = $3"
	if tx == nil {
		_, err = db.DB.Exec(query, group.State, group.UpdatedAt, group.ID)
	} else {
		_, err = tx.Exec(query, group.State, group.UpdatedAt, group.ID)
	}
	if err != nil {
		logger.Error("groups.IncrementgroupState: updateing group", rz.Err(err),
			rz.String("group.id", group.ID.String()))
		return NewError(ErrorInternal)
	}

	return nil
}
