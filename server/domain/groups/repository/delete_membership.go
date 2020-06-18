package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) DeleteMembership(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (err error) {
	return
}

// 	queryDeleteMembership := "DELETE FROM groups_members WHERE user_id = $1 AND group_id = $2"
