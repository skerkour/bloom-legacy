package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/api/apiutil"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/server/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/server/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/server/domain/objects"
)

// AcceptGroupInvitation accepts a group invitaiton
func (r *Resolver) AcceptGroupInvitation(ctx context.Context, input model.AcceptGroupInvitationInput) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.AcceptInvitationParams{
		InvitationID:       input.InvitationID,
		EncryptedMasterKey: input.EncryptedMasterKey,
		MasterKeyNonce:     input.MasterKeyNonce,
	}
	group, err := groups.AcceptInvitation(ctx, currentUser, params)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	state := objects.EncodeState(group.State)
	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
		State:       &state,
	}
	return ret, nil
}
