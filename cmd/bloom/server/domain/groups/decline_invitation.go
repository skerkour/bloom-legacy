package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
)

func DeclineInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) error {
	logger := rz.FromCtx(ctx)
	var err error

	// validate action
	if user.ID != invitation.InviteeID {
		return NewError(ErrorInvitationNotFound)
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		logger.Error("groups.DeclineInvitation: deleting invitation", rz.Err(err))
		return NewError(ErrorDecliningInvitation)
	}
	return nil
}
