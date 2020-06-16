package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func DeclineInvitation(ctx context.Context, actor *users.User, invitationID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)
	var invitation Invitation

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.DeclineInvitation: Starting transaction", rz.Err(err))
		err = NewError(ErrorDecliningInvitation)
		return
	}

	queryGetInvitation := "SELECT * FROM groups_invitations WHERE id = $1 FOR UPDATE"
	err = tx.Get(&invitation, queryGetInvitation, invitationID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.DeclineInvitation: fetching invitation", rz.Err(err),
			rz.String("invitation.id", invitationID.String()))
		err = NewError(ErrorInvitationNotFound)
		return
	}

	// validate action
	if actor.ID != invitation.InviteeID {
		tx.Rollback()
		return NewError(ErrorInvitationNotFound)
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.DeclineInvitation: deleting invitation", rz.Err(err))
		err = NewError(ErrorDecliningInvitation)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("mutation.DeclineGroupInvitation: Committing transaction", rz.Err(err))
		err = NewError(ErrorDecliningInvitation)
		return
	}

	return
}
