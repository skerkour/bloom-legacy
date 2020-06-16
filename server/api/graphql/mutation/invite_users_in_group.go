package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/groups"
)

// InviteUserInGroup is used by groups' admin to invite users in a group, by their usernames
func (r *Resolver) InviteUserInGroup(ctx context.Context, input model.InviteUserInGroupInput) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.InviteUserParams{
		GroupID:            input.GroupID,
		Username:           input.Username,
		EphemeralPublicKey: input.EphemeralPublicKey,
		Signature:          input.Signature,
		EncryptedMasterKey: input.EncryptedMasterKey,
	}
	group, err := groups.InviteUser(ctx, currentUser, params)
	if err != nil {
		err = gqlerrors.New(err)
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
