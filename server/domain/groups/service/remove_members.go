package service

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *GroupsService) RemoveMembers(ctx context.Context, params groups.RemoveMembersParams) (ret groups.Group, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	if len(params.Usernames) != 1 {
		err = errors.InvalidArgument("Can't remove more than one member")
		return
	}

	username := strings.ToLower(params.Usernames[0])
	username = strings.TrimSpace(username)

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, params.GroupID)
	if err != nil {
		return
	}

	memberToBeremoved, err := service.usersService.FindUserByUsername(ctx, username)
	if err != nil {
		return
	}

	ret, err = service.groupsRepo.FindGroupByID(ctx, service.db, params.GroupID)
	if err != nil {
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.RemoveMembers: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	err = service.groupsRepo.DeleteMembership(ctx, tx, memberToBeremoved.ID, params.GroupID)
	if err != nil {
		tx.Rollback()
		return
	}

	remainingAdmins, err := service.groupsRepo.GetGroupAdminsCount(ctx, tx, params.GroupID)
	if err != nil {
		tx.Rollback()
		return
	}

	if remainingAdmins == 0 {
		tx.Rollback()
		err = groups.ErrAtLeastOneAdministratorShouldRemainsInGroup
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "groups.RemoveMembers: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
