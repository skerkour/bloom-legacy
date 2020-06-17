package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
	"gitlab.com/bloom42/bloom/server/db"
)

func (service *GroupsService) CheckUserIsGroupAdmin(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (err error) {
	return
}
