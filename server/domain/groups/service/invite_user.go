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

func (service *GroupsService) InviteUser(ctx context.Context, params groups.InviteUserParams) (ret groups.Group, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	// clean and validate data
	params.Username = strings.TrimSpace(params.Username)
	params.Username = strings.ToLower(params.Username)

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, params.GroupID)
	if err != nil {
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.InviteUser: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	ret, err = service.groupsRepo.FindGroupByID(ctx, tx, params.GroupID)
	if err != nil {
		tx.Rollback()
		return
	}

	invitee, err := service.usersService.FindUserByUsername(ctx, params.Username)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.CheckUserIsGroupMember(ctx, tx, invitee.ID, ret.ID)
	if err != nil {
		// if membership not found, it's good so we don't return
		if _, ok := err.(*errors.NotFoundError); !ok {
			tx.Rollback()
			return
		}
	}

	// create invitation
	now := time.Now().UTC()
	invitation := groups.Invitation{
		ID:                 uuid.New(),
		CreatedAt:          now,
		UpdatedAt:          now,
		EphemeralPublicKey: params.EphemeralPublicKey,
		Signature:          params.Signature,
		EncryptedMasterKey: params.EncryptedMasterKey,
		GroupID:            ret.ID,
		InviteeID:          invitee.ID,
		InviterID:          me.ID,
	}
	err = service.groupsRepo.CreateInvitation(ctx, tx, invitation)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "groups.AcceptInvitation: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
