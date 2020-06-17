package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/sync"
)

// CreateGroup is used to create a group
func (resolver *Resolver) CreateGroup(ctx context.Context, input model.CreateGroupInput) (ret *model.Group, err error) {
	params := groups.CreateGroupParams{
		Name:               input.Name,
		Description:        input.Description,
		EncryptedMasterKey: input.EncryptedMasterKey,
		MasterKeyNonce:     input.MasterKeyNonce,
	}
	group, err := resolver.groupsService.CreateGroup(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	state := sync.EncodeState(group.State)
	ret = &model.Group{
		ID:          &group.ID,
		Name:        group.Name,
		Description: group.Description,
		CreatedAt:   &group.CreatedAt,
		State:       &state,
	}
	return
}
