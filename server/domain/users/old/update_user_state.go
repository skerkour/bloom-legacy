package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/gobox/rz"
)

// UpdateUserState update the user's state
func UpdateUserState(ctx context.Context, tx *sqlx.Tx, user *User, newState int64) error {
	var err error
	logger := rz.FromCtx(ctx)

	if user.State > newState {
		return NewError(ErrorInvalidState)
	}

	user.State = newState
	user.UpdatedAt = time.Now().UTC()

	query := "UPDATE users SET state = $1, updated_at = $2 WHERE id = $3"
	if tx == nil {
		_, err = db.DB.Exec(query, user.State, user.UpdatedAt, user.ID)
	} else {
		_, err = tx.Exec(query, user.State, user.UpdatedAt, user.ID)
	}
	if err != nil {
		logger.Error("users.IncrementUserState: updateing user", rz.Err(err),
			rz.String("user.id", user.ID.String()))
		return NewError(ErrorInternal)
	}

	return nil
}
