package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/domain/groups"
)

// InviteUserInGroup is used by groups' admin to invite users in a group, by their usernames
func (resolver *Resolver) InviteUserInGroup(ctx context.Context, input model.InviteUserInGroupInput) (ret *model.Group, err error) {
	params := groups.InviteUserParams{
		GroupID:            input.GroupID,
		Username:           input.Username,
		EphemeralPublicKey: input.EphemeralPublicKey,
		Signature:          input.Signature,
		EncryptedMasterKey: input.EncryptedMasterKey,
	}
	group, err := resolver.groupsService.InviteUser(ctx, params)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
	}
	return
}
