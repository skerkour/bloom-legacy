package service

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
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

/*
	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("mutation.CreateGroup: Starting transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}

	params := groups.CreateGroupParams{
		Name:               input.Name,
		Description:        input.Description,
		EncryptedMasterKey: input.EncryptedMasterKey,
		MasterKeyNonce:     input.MasterKeyNonce,
	}
	newGroup, err := groups.CreateGroup(ctx, tx, currentUser, params)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	// create customer profile
	_, err = billing.CreateCustomer(ctx, tx, currentUser, nil, &newGroup.ID)
	if err != nil {
		tx.Rollback()
		err = gqlerrors.New(err)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.CreateGroup: Committing transaction", rz.Err(err))
		err = gqlerrors.New(groups.NewError(groups.ErrorCreatingGroup))
		return
	}
*/
