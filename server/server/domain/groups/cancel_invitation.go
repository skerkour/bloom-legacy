package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

type CancelInvitationParams struct {
	InvitationID uuid.UUID
}

func CancelInvitation(ctx context.Context, actor *users.User, params CancelInvitationParams) error {
	logger := rz.FromCtx(ctx)
	var err error
	var invitation Invitation

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.CancelInvitation: Starting transaction", rz.Err(err))
		return NewError(ErrorCancelingInvitation)
	}

	queryGetInvitation := "SELECT * FROM groups_invitations WHERE id = $1 FOR UPDATE"
	err = tx.Get(&invitation, queryGetInvitation, params.InvitationID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.CancelInvitation: fetching invitation", rz.Err(err),
			rz.String("invitation.id", params.InvitationID.String()))
		return NewError(ErrorInvitationNotFound)
	}

	// verify that user is admin
	if err = CheckUserIsGroupAdmin(ctx, tx, actor.ID, invitation.GroupID); err != nil {
		tx.Rollback()
		return err
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.CancelInvitation: deleting invitation", rz.Err(err))
		return NewError(ErrorCancelingInvitation)
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.CancelInvitation: Committing transaction", rz.Err(err))
		return NewError(ErrorCancelingInvitation)
	}

	return nil
}
