package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/apiutil"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/gqlerrors"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/api/graphql/model"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/groups"
)

func (r *Resolver) CancelGroupInvitation(ctx context.Context, input model.CancelGroupInvitationInput) (ret bool, err error) {
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	params := groups.CancelInvitationParams{
		InvitationID: input.ID,
	}
	err = groups.CancelInvitation(ctx, currentUser, params)
	if err != nil {
		return ret, gqlerrors.New(err)
	}

	ret = true
	return ret, nil
}
