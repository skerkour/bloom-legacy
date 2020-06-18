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
	// query := `SELECT DISTINCT ON (invitation_id) groups_invitations.id invitation_id, groups_invitations.group_id invitation_group_id,
	// groups_invitations.created_at invitation_created_at, inviters.*, invitees.*
	// 		FROM
	// 			(SELECT
	// 				id inviter_id, users.username inviter_username,
	// 				users.display_name inviter_display_name, users.avatar_id inviter_avatar_id
	// 				FROM users
	// 			) AS inviters,
	// 			(SELECT
	// 				id invitee_id, users.username invitee_username,
	// 				users.display_name invitee_display_name, users.avatar_id invitee_avatar_id
	// 				FROM users
	// 			) AS invitees
	// 			INNER JOIN groups_invitations
	// 				ON groups_invitations.inviter_id = inviter_id
	// 				OR groups_invitations.invitee_id = invitees.invitee_id
	// 		WHERE groups_invitations.group_id = $1
	// 		`
	query := `SELECT DISTINCT ON (invitation_id) groups_invitations.id invitation_id, groups_invitations.group_id invitation_group_id,
	groups_invitations.created_at invitation_created_at,
	inviters.id inviter_id, inviters.username inviter_username, inviters.display_name inviter_display_name, inviters.avatar inviter_avatar,
	invitees.id invitee_id, invitees.username invitee_username, invitees.display_name invitee_display_name, invitees.avatar invitee_avatar
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
