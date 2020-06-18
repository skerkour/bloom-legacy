package query

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/sync"
	"gitlab.com/bloom42/gobox/uuid"
)

// Group find a group
func (resolver *Resolver) Group(ctx context.Context, groupID uuid.UUID) (ret *model.Group, err error) {
	group, err := resolver.groupsService.FindGroupById(ctx, groupID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	state := sync.EncodeState(group.State)
	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
		State:       &state,
	}
	return
}
