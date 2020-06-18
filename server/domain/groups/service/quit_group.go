package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) QuitGroup(ctx context.Context, groupID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	err = service.CheckUserIsGroupMember(ctx, service.db, me.ID, groupID)
	if err != nil {
		return err
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.QuitGroup: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	err = service.groupsRepo.DeleteMembership(ctx, tx, me.ID, groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	remainingAdmins, err := service.groupsRepo.GetGroupAdminsCount(ctx, tx, groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	if remainingAdmins == 0 {
		err = groups.ErrAtLeastOneAdministratorShouldRemainsInGroup
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "groups.QuitGroup: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
