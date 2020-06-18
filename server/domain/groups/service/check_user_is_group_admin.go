package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) CheckUserIsGroupAdmin(ctx context.Context, db db.Queryer, userID, groupID uuid.UUID) (err error) {
	membership, err := service.groupsRepo.FindMembershipForUser(ctx, db, userID, groupID)
	if err != nil {
		return
	}
	if membership.Role != groups.RoleAdministrator {
		err = groups.ErrAdminRoleRequired
	}
	return
}
