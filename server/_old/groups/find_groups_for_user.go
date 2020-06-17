package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupsForUser(ctx context.Context, tx *sqlx.Tx, userID uuid.UUID) ([]Group, error) {
	ret := []Group{}
	var err error
	logger := rz.FromCtx(ctx)

	query := `SELECT groups.* FROM groups
		INNER JOIN groups_members ON groups_members.group_id = groups.id
		WHERE groups_members.user_id = $1`
	if tx == nil {
		err = db.DB.Select(&ret, query, userID)
	} else {
		err = tx.Select(&ret, query, userID)
	}
	if err != nil {
		logger.Error("groups.FindGroupsForUser: findings groups", rz.Err(err),
			rz.String("group.id", userID.String()))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
