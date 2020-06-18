package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindInvitationsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []groups.UserInvitation, err error) {
	ret = []groups.UserInvitation{}
	query := `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
	invit.encrypted_master_key AS invitation_encrypted_master_key,
	invit.ephemeral_public_key AS invitation_ephemeral_public_key, invit.signature AS invitation_signature,
	groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
		users.username AS inviter_username, users.display_name AS inviter_display_name, users.public_key AS inviter_public_key
		FROM groups_invitations AS invit, groups, users
		WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindInvitationsForUser: finding invitations"
		logger.Error(errMessage, log.Err("error", err), log.UUID("user.id", userID))
		err = errors.Internal(errMessage, err)
	}
	return
}
