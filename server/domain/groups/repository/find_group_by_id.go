package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindGroupByID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret groups.Group, err error) {
	return
}
