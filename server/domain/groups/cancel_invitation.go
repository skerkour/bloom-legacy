package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func CancelInvitation(ctx context.Context, tx *sqlx.Tx, user users.User, invitation Invitation) error {
	logger := rz.FromCtx(ctx)
	var err error

	// verify that user is admin
	if err = CheckUserIsGroupAdmin(ctx, tx, user.ID, invitation.GroupID); err != nil {
		return err
	}

	// delete invitation
	queryDeleteInvitation := "DELETE FROM groups_invitations WHERE id = $1"
	_, err = tx.Exec(queryDeleteInvitation, invitation.ID)
	if err != nil {
		logger.Error("groups.CancelInvitation: creating membership", rz.Err(err))
		return NewError(ErrorCancelingInvitation)
	}
	return nil
}
