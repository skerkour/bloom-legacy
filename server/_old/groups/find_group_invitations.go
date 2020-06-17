package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupInvitations(ctx context.Context, tx *sqlx.Tx, groupId uuid.UUID) ([]InvitationPopulated, error) {
	ret := []InvitationPopulated{}
	var err error
	logger := rz.FromCtx(ctx)

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
	inviters.id inviter_id, inviters.username inviter_username, inviters.display_name inviter_display_name, inviters.avatar_id inviter_avatar_id,
	invitees.id invitee_id, invitees.username invitee_username, invitees.display_name invitee_display_name, invitees.avatar_id invitee_avatar_id
		FROM groups_invitations

			INNER JOIN users AS inviters ON (groups_invitations.inviter_id = inviters.id)
			INNER JOIN users AS invitees ON (groups_invitations.invitee_id = invitees.id)
		WHERE groups_invitations.group_id = $1
	`

	if tx == nil {
		err = db.DB.Select(&ret, query, groupId)
	} else {
		err = tx.Select(&ret, query, groupId)
	}
	if err != nil {
		logger.Error("finding group invitations", rz.Err(err),
			rz.String("group.id", groupId.String()))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
