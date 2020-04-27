package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// FindAllSessionsForUserID finds all the sessions for a given userID
func FindAllSessionsForUserID(ctx context.Context, tx *sqlx.Tx, userID uuid.UUID) ([]Session, error) {
	ret := []Session{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM sessions WHERE user_id = $1"
	if tx == nil {
		err = db.DB.Select(&ret, queryFind, userID)
	} else {
		err = tx.Select(&ret, queryFind, userID)
	}
	if err != nil {
		logger.Error("users.FindAllSessionsForUserID: finding sessions", rz.Err(err),
			rz.String("user.id", userID.String()))
		return ret, NewError(ErrorSessionNotFound)
	}

	return ret, err
}
