package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

// TODO: delete group's files
func (service *GroupsService) DeleteGroup(ctx context.Context, groupID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, groupID)
	if err != nil {
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.DeleteGroup: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	customer, err := service.billingRepo.FindCustomerByGroupID(ctx, tx, groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	if customer.UsedStorage != 0 {
		tx.Rollback()
		err = errors.InvalidArgument("Please delete all files and data before deleting group")
		return
	}

	err = service.billingService.DetachCustomer(ctx, tx, me, &groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.groupsRepo.DeleteGroup(ctx, tx, groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.syncRepo.DeleteGroupObjects(ctx, tx, groupID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "groups.DeleteGroup: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	return
}
