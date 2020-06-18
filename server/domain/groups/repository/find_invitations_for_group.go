package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindInvitationsForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []groups.GroupInvitation, err error) {
	ret = []groups.GroupInvitation{}
	return
}
