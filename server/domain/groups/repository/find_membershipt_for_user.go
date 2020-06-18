package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindMembershipForUser(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (ret groups.Membership, err error) {
	return
}
