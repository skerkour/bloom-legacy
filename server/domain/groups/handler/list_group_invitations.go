package handler

import (
	"context"
	"time"

	"github.com/twitchtv/twirp"
	rpc "gitlab.com/bloom42/bloom/common/rpc/groups"
	"gitlab.com/bloom42/bloom/server/api/apictx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/libs/rz-go"
)

func (handler Handler) ListGroupInvitations(ctx context.Context, params *rpc.ListGroupInvitationsParams) (*rpc.InvitationList, error) {
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

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.ListGroupInvitations: Starting transaction", rz.Err(err))
		return ret, twirp.InternalError("internal error")
	}

	if twerr := groups.CheckUserIsGroupAdmin(ctx, tx, apiCtx.AuthenticatedUser.ID, params.GroupId); twerr != nil {
		tx.Rollback()
		return ret, twerr
	}

	invitations := []invit{}
	err = tx.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
		users.username AS inviter_username, users.display_name AS inviter_display_name
		FROM groups_invitations AS invit, groups, users
		WHERE invit.group_id = $1 AND users.id = invit.inviter_id`, params.GroupId)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.ListGroupInvitations: fetching invitations", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.ListGroupInvitations: Committing transaction", rz.Err(err))
		return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	}

	for _, invitation := range invitations {
		rpcInvitation := rpc.Invitation{
			Id: invitation.ID,
			Group: &rpc.Group{
				Id:          invitation.GroupID,
				CreatedAt:   invitation.GroupCreatedAt.Format(time.RFC3339),
				Name:        invitation.GroupName,
				Description: invitation.GroupDescription,
			},
			Inviter: &rpc.Inviter{
				Username:    invitation.InviterUsername,
				DisplayName: invitation.InviterDisplayName,
			},
		}
		ret.Invitations = append(ret.Invitations, &rpcInvitation)
	}
	return ret, nil
}
