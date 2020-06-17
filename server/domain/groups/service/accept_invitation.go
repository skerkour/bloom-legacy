package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (service *GroupsService) AcceptInvitation(ctx context.Context, params groups.AcceptInvitationParams) (ret groups.Group, err error) {
	return
}
