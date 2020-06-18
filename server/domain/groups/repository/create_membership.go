package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) CreateMembership(ctx context.Context, db db.Queryer, membership groups.Membership) (err error) {
	return
}
