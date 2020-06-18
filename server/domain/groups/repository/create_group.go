package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (repo *GroupsRepository) CreateGroup(ctx context.Context, db db.Queryer, group groups.Group) (err error) {
	query := `INSERT INTO groups
	(id, created_at, updated_at, name, description, avatar_id, state)
	VALUES ($1, $2, $3, $4, $5, $6, $7)`

	_, err = db.Exec(ctx, query, group.ID, group.CreatedAt, group.UpdatedAt, group.Name,
		group.Description, group.AvatardID, group.State)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.CreateGroup: inserting group"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", group.ID))
		err = errors.Internal(errMessage, err)
	}
	return
}
