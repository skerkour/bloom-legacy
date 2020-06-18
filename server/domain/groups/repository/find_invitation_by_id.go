package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindInvitationByID(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (ret groups.Invitation, err error) {
	return
}
