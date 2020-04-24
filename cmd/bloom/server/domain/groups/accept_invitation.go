package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
)

func AcceptInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) (*Group, error) {
	logger := rz.FromCtx(ctx)
	var err error
	var ret *Group

	// validate action
	if user.ID.String() != invitation.InviteeID {
		return ret, NewError(ErrorInvitationNotFound)
	}

	membership := Membership{
		JoinedAt:  time.Now().UTC(),
		GroupID:   invitation.GroupID,
		UserID:    user.ID.String(),
		Role:      RoleMember,
		InviterID: invitation.InviterID,
	}

	// create membership
	queryCreateMembership := `INSERT INTO groups_members
		(joined_at, inviter_id, group_id, user_id, role)
		VALUES ($1, $2, $3, $4, $5)`
	_, err = tx.Exec(queryCreateMembership, membership.JoinedAt, membership.InviterID, membership.GroupID,
		membership.UserID, membership.Role)
	if err != nil {
		logger.Error("creating membership", rz.Err(err))
		return ret, NewError(ErrorAcceptingInvitation)
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		logger.Error("creating membership", rz.Err(err))
		return ret, NewError(ErrorInvitationNotFound)
	}

	ret, err = FindGroupById(ctx, tx, membership.GroupID)
	return ret, err
}
