package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) CreateGroup(ctx context.Context, db db.Queryer, group groups.Group) (err error) {
	return
}
