package model

import (
	"context"
	"errors"
	"time"
)

type Group struct {
	ID          *string    `json:"id"`
	CreatedAt   *time.Time `json:"createdAt"`
	Name        string     `json:"name"`
	Description string     `json:"description"`
}

type GroupResolver struct{}

func (r *GroupResolver) Members(ctx context.Context, obj *Group) (*GroupMemberConnection, error) {
	return nil, errors.New("Forbidden")
}
func (r *GroupResolver) Invitations(ctx context.Context, obj *Group) (*GroupInvitationConnection, error) {
	var ret *GroupInvitationConnection
	// logger := rz.FromCtx(ctx)
	// apiCtx, ok := ctx.Value(apictx.Key).(*apictx.Context)
	// if !ok {
	// 	return ret, twirp.InternalError("internal error")
	// }
	// if apiCtx.AuthenticatedUser == nil {
	// 	twerr := twirp.NewError(twirp.Unauthenticated, "authentication required")
	// 	return ret, twerr
	// }

	// tx, err := db.DB.Beginx()
	// if err != nil {
	// 	logger.Error("groups.ListGroupInvitations: Starting transaction", rz.Err(err))
	// 	return ret, twirp.InternalError("internal error")
	// }

	// if twerr := groups.CheckUserIsGroupAdmin(ctx, tx, apiCtx.AuthenticatedUser.ID, params.GroupId); twerr != nil {
	// 	tx.Rollback()
	// 	return ret, twerr
	// }

	// invitations := []invit{}
	// err = tx.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	// groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
	// 	users.username AS inviter_username, users.display_name AS inviter_display_name
	// 	FROM groups_invitations AS invit, groups, users
	// 	WHERE invit.group_id = $1 AND users.id = invit.inviter_id`, params.GroupId)
	// if err != nil {
	// 	tx.Rollback()
	// 	logger.Error("groups.ListGroupInvitations: fetching invitations", rz.Err(err))
	// 	return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	// }

	// err = tx.Commit()
	// if err != nil {
	// 	tx.Rollback()
	// 	logger.Error("groups.ListGroupInvitations: Committing transaction", rz.Err(err))
	// 	return ret, twirp.InternalError(groups.ErrorListingInvitationsMsg)
	// }

	// for _, invitation := range invitations {
	// 	ret.Invitations = append(ret.Invitations, invitToRpcInvitation(invitation))
	// }
	return ret, nil
}
