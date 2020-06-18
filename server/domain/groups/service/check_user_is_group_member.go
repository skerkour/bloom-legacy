package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) CheckUserIsGroupMember(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (err error) {
	_, err = service.groupsRepo.FindMembershipForUser(ctx, db, userID, groupID)
	return
}
