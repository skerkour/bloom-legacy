package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (repo *GroupsRepository) (ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []Group, err error) {
	ret = []groups.Group{}
	return
}
