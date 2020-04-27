package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

// AcceptGroupInvitation accepts a group invitaiton
func (r *Resolver) AcceptGroupInvitation(ctx context.Context, input model.AcceptGroupInvitationInput) (ret *model.Group, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.AcceptInvitationParams{
		InvitationID: input.ID,
	}
	group, err := groups.AcceptInvitation(ctx, currentUser, params)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = &model.Group{
		ID:          &group.ID,
		CreatedAt:   &group.CreatedAt,
		Name:        group.Name,
		Description: group.Description,
		AvatarURL:   nil,
	}
	return ret, nil
}
