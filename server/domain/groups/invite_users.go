package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func InviteUsers(ctx context.Context, tx *sqlx.Tx, inviter users.User, group Group, usernames []string) error {
	logger := rz.FromCtx(ctx)
	inviteesIds := []string{}
	var err error

	queryInviteesIds := "SELECT id FROM users WHERE username IN ($1)"
	err = tx.Get(&inviteesIds, queryInviteesIds, usernames)
	if err != nil {
		logger.Error("groups.InviteUsers: error fetching invitees ids", rz.Err(err))
		return NewError(ErrorInvitingUsers)
	}
	if len(inviteesIds) != len(usernames) {
		return NewError(ErrorUsernamesNotFound)
	}

	if err = validateInviteUsers(ctx, tx, inviter, group, inviteesIds); err != nil {
		return err
	}

	return nil
}

func validateInviteUsers(ctx context.Context, tx *sqlx.Tx, inviter users.User, group Group, inviteesIds []string) error {
	logger := rz.FromCtx(ctx)
	var alreadyInUsers int
	var err error

	// check that inviter inviting is admin
	if err = CheckUserIsGroupAdmin(ctx, tx, inviter.ID, group.ID); err != nil {
		return err
	}

	//  check that user is not already in group or awaiting invitations
	queryAlreadyInGroupOrInvitations := `SELECT COUNT(*)
		FROM groups_members, groups_invitations
		WHERE (groups_members.group_id = $1 AND groups_members.user_id IN ($2))
			OR (groups_invitations.invitee_id IN ($2))`
	err = tx.Get(&alreadyInUsers, queryAlreadyInGroupOrInvitations, group.ID, inviteesIds)
	if err != nil {
		logger.Error("groups.InviteUsers: error fetching users already in group or invitations", rz.Err(err))
		return NewError(ErrorInvitingUsers)
	}
	if alreadyInUsers != 0 {
		return NewError(ErrorUserAlreadyInGroup)
	}

	return nil
}
