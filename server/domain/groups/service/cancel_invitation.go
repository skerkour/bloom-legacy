package service

import (
	"context"

	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) CancelInvitation(ctx context.Context, invitationID uuid.UUID) (err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}

	invitation, err := service.groupsRepo.FindInvitationByID(ctx, service.db, invitationID)
	if err != nil {
		return
	}

	err = service.CheckUserIsGroupAdmin(ctx, service.db, me.ID, invitation.GroupID)
	if err != nil {
		return
	}

	err = service.groupsRepo.DeleteInvitation(ctx, service.db, invitation.ID)
	return
}
