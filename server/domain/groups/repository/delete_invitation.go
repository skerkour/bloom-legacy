package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) DeleteInvitation(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (err error) {
	query := "DELETE FROM groups_invitations WHERE id = $1"

	_, err = db.Exec(ctx, query, invitationID)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.DeleteInvitation: deleting invitation"
		logger.Error(errMessage, log.Err("error", err), log.UUID("invitation.id", invitationID))
		err = errors.Internal(errMessage, err)
	}
	return
}
