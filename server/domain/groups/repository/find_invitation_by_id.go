package repository

import (
	"context"
	"database/sql"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindInvitationByID(ctx context.Context, db db.Queryer, invitationID uuid.UUID) (ret groups.Invitation, err error) {
	query := "SELECT * FROM groups_invitations WHERE id = $1"

	err = db.Get(ctx, &ret, query, groupID)
	if err != nil {
		if err == sql.ErrNoRows {
			err = errors.NotFound("Invitation not found")
		} else {
			logger := log.FromCtx(ctx)
			const errMessage = "groups.FindInvitationByID: finding invitation"
			logger.Error(errMessage, log.Err("error", err), log.UUID("invitation.id", invitationID))
			err = errors.Internal(errMessage, err)
		}
	}
	return
}
