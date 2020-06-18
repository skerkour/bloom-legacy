package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *UsersRepository) FindAllSessionsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []users.Session, err error) {
	ret = []users.Session{}
	query := "SELECT * FROM sessions WHERE user_id = $1 ORDER BY created_at"

	err = db.Select(ctx, &ret, query, userID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindAllSessionsForUser: finding sessions"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return
}
