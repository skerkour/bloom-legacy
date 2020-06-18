package repository

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (repo *GroupsRepository) FindInvitationsForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []groups.GroupInvitation, err error) {
	ret = []groups.GroupInvitation{}
	query := `SELECT DISTINCT ON (invitation_id) groups_invitations.id invitation_id, groups_invitations.group_id invitation_group_id,
	groups_invitations.created_at invitation_created_at,
	inviters.id inviter_id, inviters.username inviter_username, inviters.display_name inviter_display_name, inviters.avatar_id inviter_avatar_id,
	invitees.id invitee_id, invitees.username invitee_username, invitees.display_name invitee_display_name, invitees.avatar_id invitee_avatar_id
		FROM groups_invitations
			INNER JOIN users AS inviters ON (groups_invitations.inviter_id = inviters.id)
			INNER JOIN users AS invitees ON (groups_invitations.invitee_id = invitees.id)
		WHERE groups_invitations.group_id = $1
	`

	err = db.Select(ctx, &ret, query)
	if err != nil {
		logger := log.FromCtx(ctx)
		const errMessage = "groups.FindInvitationsForGroup: finding invitations"
		logger.Error(errMessage, log.Err("error", err), log.UUID("group.id", groupID))
		err = errors.Internal(errMessage, err)
	}
	return
}
