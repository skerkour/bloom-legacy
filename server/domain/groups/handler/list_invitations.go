package handler

import (
	"context"

	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

func (handler Handler) ListInvitations(ctx context.Context, _ *rpc.Empty) (*rpc.InvitationList, error) {
	ret := &rpc.InvitationList{Invitations: []*rpc.Invitation{}}
	logger := rz.FromCtx(ctx)
	apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	if !ok {
		return ret, twirp.InternalError("internal error")
	}
	if apiCtx.AuthenticatedUser == nil {
		twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
		return ret, twerr
	}

	invitations := []invit{}
	err := db.DB.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
		users.username AS inviter_username, users.display_name AS inviter_display_name
		FROM groups_invitations AS invit, groups, users
		WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`, apiCtx.AuthenticatedUser.ID)
	if err != nil {
		logger.Error("groups.ListGroups: fetching invitations", rz.Err(err))
		return ret, twirp.InternalError("Internal error fetching invitations. Please try again.")
	}

	for _, invitation := range invitations {
		ret.Invitations = append(ret.Invitations, invitToRpcInvitation(invitation))
	}
	return ret, nil
}
