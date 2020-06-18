package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *GroupsRepository) UpdateGroup(ctx context.Context, db db.Queryer, group groups.Group) (err error) {
	query := `UPDATE groups SET
	updated_at = $1, name = $2, description = $3, avatar_id = $4, state = $5
	WHERE id = $6`

	_, err = db.Exec(ctx, query, group.UpdatedAt, group.Name, group.Description, group.AvatardID,
		group.State, group.ID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.UpdateGroup: updating group"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", group.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
