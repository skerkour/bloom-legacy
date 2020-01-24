package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func AcceptInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error

	// validate action
	if user.ID != invitation.InviteeID {
		return twirp.NewError(twirp.NotFound, "Invitation not found.")
	}

	membership := Membership{
		CreatedAt: time.Now().UTC(),
		GroupID:   invitation.GroupID,
		UserID:    user.ID,
		Role:      RoleMember,
	}

	// create membership
	queryCreateMembership := `INSERT INTO groups_members
		(created_at, group_id, user_id, role)
		VALUES ($1, $2, $3, $4)`
	_, err = tx.Exec(queryCreateMembership, membership.CreatedAt, membership.GroupID,
		membership.UserID, membership.Role)
	if err != nil {
		logger.Error("groups.AcceptInvitation: creating membership", rz.Err(err))
		return twirp.InternalError(ErrorAcceptingInvitationMsg)
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		logger.Error("groups.AcceptInvitation: creating membership", rz.Err(err))
		return twirp.InternalError(ErrorAcceptingInvitationMsg)
	}
	return nil
}
