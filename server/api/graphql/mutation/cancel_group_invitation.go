package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// CancelGroupInvitation is used by groups' admins to cancel a group invitation
func (resolver *Resolver) CancelGroupInvitation(ctx context.Context, input model.CancelGroupInvitationInput) (ret bool, err error) {
	err = resolver.groupsService.CancelInvitation(ctx, input.InvitationID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
