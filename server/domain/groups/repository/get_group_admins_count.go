package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) GetGroupAdminsCount(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret int64, err error) {
	return
}

// "SELECT COUNT(*) FROM groups_members WHERE group_id = $1 AND role = $2"
