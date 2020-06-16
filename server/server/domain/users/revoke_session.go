package users

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// RevokeSession revokes a given session
func RevokeSession(ctx context.Context, actor *User, sessionID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("users.RevokeSession: Starting transaction", rz.Err(err))
		err = NewError(ErrorDeletingSession)
		return
	}

	err = deleteSession(ctx, tx, sessionID, actor.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("users.RevokeSession: committing transaction", rz.Err(err))
		err = NewError(ErrorDeletingSession)
		return
	}

	globalSessionsCache.Delete(sessionID)

	return
}
