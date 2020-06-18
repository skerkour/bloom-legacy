package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) FindAllGroups(ctx context.Context, db db.Queryer) (ret []groups.Group, err error) {
	ret = []groups.Group{}
	return
}
