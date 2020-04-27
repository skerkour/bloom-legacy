package groups

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type AcceptInvitationParams struct {
	InvitationID uuid.UUID
}

func AcceptInvitation(ctx context.Context, actor *users.User, params AcceptInvitationParams) (ret *Group, err error) {
	logger := rz.FromCtx(ctx)
	var invitation Invitation

	// start transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.AcceptInvitation: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorAcceptingInvitation)
	}

	queryGetInvitation := "SELECT * FROM groups_invitations WHERE id = $1 FOR UPDATE"
	err = tx.Get(&invitation, queryGetInvitation, params.InvitationID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.AcceptInvitation: fetching invitation", rz.Err(err),
			rz.String("invitation.id", params.InvitationID.String()))
		return ret, NewError(ErrorAcceptingInvitation)
	}

	// validate
	if actor.ID != invitation.InviteeID {
		tx.Rollback()
		return ret, NewError(ErrorInvitationNotFound)
	}

	membership := Membership{
		JoinedAt:  time.Now().UTC(),
		GroupID:   invitation.GroupID,
		UserID:    actor.ID,
		Role:      consts.GROUP_ROLE_MEMBER,
		InviterID: invitation.InviterID,
	}

	// create membership
	queryCreateMembership := `INSERT INTO groups_members
		(joined_at, inviter_id, group_id, user_id, role)
		VALUES ($1, $2, $3, $4, $5)`
	_, err = tx.Exec(queryCreateMembership, membership.JoinedAt, membership.InviterID, membership.GroupID,
		membership.UserID, membership.Role)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.AcceptInvitation: creating membership", rz.Err(err))
		return ret, NewError(ErrorAcceptingInvitation)
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.AcceptInvitation: creating membership", rz.Err(err))
		return ret, NewError(ErrorInvitationNotFound)
	}

	ret, err = FindGroupById(ctx, tx, membership.GroupID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.AcceptInvitation: finding group", rz.Err(err),
			rz.String("group.id", membership.GroupID.String()))
		return ret, NewError(ErrorInvitationNotFound)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.AcceptGroupInvitation: Committing transaction", rz.Err(err))
		return ret, NewError(ErrorAcceptingInvitation)
	}

	return ret, err
}
