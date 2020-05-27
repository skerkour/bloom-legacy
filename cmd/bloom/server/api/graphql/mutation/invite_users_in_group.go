package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

// InviteUsersInGroup is used by groups' admin to invite users in a group, by their usernames
func (r *Resolver) InviteUserInGroup(ctx context.Context, input model.InviteUserInGroupInput) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.InviteUserParams{
		GroupID:                     input.GroupID,
		Username:                    input.Username,
		EphemeralPublicKey:          input.EphemeralPublicKey,
		InvitationSignature:         input.InvitationSignature,
		EncryptedMasterKey:          input.EncryptedMasterKey,
		EncryptedMasterKeySignature: input.EncryptedMasterKeySignature,
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
