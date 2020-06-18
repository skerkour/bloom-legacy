package service

import (
	"context"
	"strings"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) CreateGroup(ctx context.Context, params groups.CreateGroupParams) (ret groups.Group, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate params
	params.Name = strings.TrimSpace(params.Name)
	params.Description = strings.TrimSpace(params.Description)
	err = validateCreateGroup(params.Name, params.Description)
	if err != nil {
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.CreateGroup: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	now := time.Now().UTC()
	ret = groups.Group{
		ID:          uuid.New(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Name:        params.Name,
		Description: params.Description,
		State:       0,
	}
	err = service.groupsRepo.CreateGroup(ctx, tx, ret)
	if err != nil {
		tx.Rollback()
		return
	}

	membership := groups.Membership{
		JoinedAt:           time.Now().UTC(),
		GroupID:            ret.ID,
		UserID:             me.ID,
		Role:               groups.RoleAdministrator,
		InviterID:          me.ID,
		EncryptedMasterKey: params.EncryptedMasterKey,
		MasterKeyNonce:     params.MasterKeyNonce,
	}
	err = service.groupsRepo.CreateMembership(ctx, tx, membership)
	if err != nil {
		tx.Rollback()
		return
	}

	_, err = service.billingService.CreateCustomer(ctx, tx, me, &ret.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "groups.CreateGroup: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
