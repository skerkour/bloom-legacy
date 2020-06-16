package users

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// FindSessionByID returns a session for the given ID
func FindSessionByID(ctx context.Context, tx *sqlx.Tx, sessionID uuid.UUID) (*Session, error) {
	ret := &Session{}
	var err error
	logger := rz.FromCtx(ctx)

	query := "SELECT * FROM sessions WHERE id = $1"
	if tx == nil {
		err = db.DB.Get(&ret, query, sessionID)
	} else {
		err = tx.Get(&ret, query, sessionID)
	}
	if err != nil {
		logger.Error("users.FindSessionById: finding session", rz.Err(err),
			rz.String("session.id", sessionID.String()))
		return ret, NewError(ErrorSessionNotFound)
	}

	return ret, err
}
