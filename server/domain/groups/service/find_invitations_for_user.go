package service

import (
	"context"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *GroupsService) FindInvitationsForUser(ctx context.Context, userID uuid.UUID) (ret []groups.UserInvitation, err error) {
	return
}

/*

	logger := rz.FromCtx(ctx)
	currentUser := apiutil.UserFromCtx(ctx)

	if currentUser == nil {
		return ret, gqlerrors.AuthenticationRequired()
	}

	if currentUser.ID != uuid.UUID(*user.ID) && !currentUser.IsAdmin {
		return ret, PermissionDeniedToAccessField()
	}

	invitations := []invit{}
	err = db.DB.Select(&invitations, `SELECT invit.id AS invitation_id, invit.created_at AS invitation_created_at,
		invit.encrypted_master_key AS invitation_encrypted_master_key,
		invit.ephemeral_public_key AS invitation_ephemeral_public_key, invit.signature AS invitation_signature,
		groups.id AS group_id, groups.created_at AS group_created_at, groups.name AS group_name, groups.description AS group_description,
			users.username AS inviter_username, users.display_name AS inviter_display_name, users.public_key AS inviter_public_key
			FROM groups_invitations AS invit, groups, users
			WHERE invit.group_id = groups.id AND invit.invitee_id = $1 AND users.id = invit.inviter_id`, user.ID)
	if err != nil {
		logger.Error("groups.ListGroups: fetching invitations", rz.Err(err))
		return ret, gqlerrors.Internal()
	}

*/
