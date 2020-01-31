package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeclineInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error

	// validate action
	if user.ID != invitation.InviteeID {
		return twirp.NewError(twirp.NotFound, "Invitation not found.")
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		logger.Error("groups.DeclineInvitation: deleting invitation", rz.Err(err))
		return twirp.InternalError(ErrorDecliningInvitationMsg)
	}
	return nil
}
