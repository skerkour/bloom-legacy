package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) DeleteGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (err error) {
	return
}

// "DELETE FROM groups WHERE id = $1"
