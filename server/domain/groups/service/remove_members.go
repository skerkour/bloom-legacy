package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
)

func (service *GroupsService) RemoveMembers(ctx context.Context, params groups.RemoveMembersParams) (ret groups.Group, err error) {
	return
}
