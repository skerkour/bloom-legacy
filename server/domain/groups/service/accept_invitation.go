package service

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *GroupsService) AcceptInvitation(ctx context.Context, params groups.AcceptInvitationParams) (ret groups.Group, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "groups.AcceptInvitation: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	invitation, err := service.groupsRepo.FindInvitationByID(ctx, tx, params.InvitationID)
	if err != nil {
		tx.Rollback()
		return
	}

	// validate
	if me.ID != invitation.InviteeID {
		tx.Rollback()
		err = groups.ErrInvitationNotFound
		return
	}

	membership := groups.Membership{
		JoinedAt:           time.Now().UTC(),
		GroupID:            invitation.GroupID,
		UserID:             me.ID,
		Role:               groups.RoleMember,
		InviterID:          invitation.InviterID,
		EncryptedMasterKey: params.EncryptedMasterKey,
		MasterKeyNonce:     params.MasterKeyNonce,
	}
	err = service.groupsRepo.CreateMembership(ctx, tx, membership)
	if err != nil {
		tx.Rollback()
		return
	}

	err = service.groupsRepo.DeleteInvitation(ctx, tx, invitation.ID)
	if err != nil {
		tx.Rollback()
		return
	}

	ret, err = service.groupsRepo.FindGroupByID(ctx, tx, membership.GroupID)
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
