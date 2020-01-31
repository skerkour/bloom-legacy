package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func InviteUsers(ctx context.Context, tx *sqlx.Tx, inviter users.User, group Group, usernames []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	inviteesIds := []string{}
	var err error

	queryInviteesIds := "SELECT id FROM users WHERE username IN ($1)"
	err = tx.Get(&inviteesIds, queryInviteesIds, usernames)
	if err != nil {
		logger.Error("groups.InviteUsers: error fetching invitees ids", rz.Err(err))
		return twirp.InternalError(ErrorInvitingUsersMsg)
	}
	if len(inviteesIds) != len(usernames) {
		return twirp.NewError(twirp.NotFound, "Some usernames were not found. Please verify your invitees list and retry.")
	}

	if twerr := validateInviteUsers(ctx, tx, inviter, group, inviteesIds); twerr != nil {
		return twerr
	}

	return nil
}

func validateInviteUsers(ctx context.Context, tx *sqlx.Tx, inviter users.User, group Group, inviteesIds []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	var alreadyInUsers int
	var err error

	// check that inviter inviting is admin
	if twerr := CheckUserIsGroupAdmin(ctx, tx, inviter.ID, group.ID); twerr != nil {
		return twerr
	}

	//  check that user is not already in group or awaiting invitations
	queryAlreadyInGroupOrInvitations := `SELECT COUNT(*)
		FROM groups_members, groups_invitations
		WHERE (groups_members.group_id = $1 AND groups_members.user_id IN ($2))
			OR (groups_invitations.invitee_id IN ($2))`
	err = tx.Get(&alreadyInUsers, queryAlreadyInGroupOrInvitations, group.ID, inviteesIds)
	if err != nil {
		logger.Error("groups.InviteUsers: error fetching users already in group or invitations", rz.Err(err))
		return twirp.InternalError(ErrorInvitingUsersMsg)
	}
	if alreadyInUsers != 0 {
		return twirp.NewError(twirp.PermissionDenied, "At least one user is already in group or invited. Please remove it and retry.")
	}

	return nil
}
