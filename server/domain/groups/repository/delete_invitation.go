package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) DeleteInvitation(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (err error) {
	return
}
