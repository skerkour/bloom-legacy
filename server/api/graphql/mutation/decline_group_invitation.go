package mutation

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api"
	"gitlab.com/bloom42/bloom/server/api/graphql/model"
)

// DeclineGroupInvitation is used to decline a group invitation
func (resolver *Resolver) DeclineGroupInvitation(ctx context.Context, input model.DeclineGroupInvitationInput) (ret bool, err error) {
	err = resolver.groupsService.DeclineInvitation(ctx, input.InvitationID)
	if err != nil {
		err = api.NewError(err)
		return
	}

	ret = true
	return
}
