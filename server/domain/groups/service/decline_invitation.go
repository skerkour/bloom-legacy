package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) DeclineInvitation(ctx context.Context, invitationID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	invitation, err := service.groupsRepo.FindInvitationByID(ctx, service.db, invitationID)
	if err != nil {
		return
	}

	// validate action
	if me.ID != invitation.InviteeID {
		err = groups.ErrInvitationNotFound
		return
	}

	err = service.groupsRepo.DeleteInvitation(ctx, service.db, invitationID)
	return
}
