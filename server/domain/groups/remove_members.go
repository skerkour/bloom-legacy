package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func RemoveMembers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error
	var remainingAdmins int

	if twerr := validateRemoveMembers(ctx, tx, user, group); err != nil {
		return twerr
	}

	// delete memberships
	queryDeleteMemberships := `DELETE FROM groups_members
		INNER JOIN users ON users.id = groups_members.user_id
		WHERE users.username IN ($1)`
	_, err = tx.Exec(queryDeleteMemberships, usernames)
	if err != nil {
		logger.Error("groups.RemoveMembers: removing members", rz.Err(err))
		return twirp.InternalError(ErrorRemovingMembersMsg)
	}

	queryRemainingAdmins := `SELECT COUNT(*) FROM groups_members
		INNER JOIN users ON users.id = groups_members.user_id
		WHERE groups_members.role = $1`
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, RoleAdministrator)
	if err != nil {
		logger.Error("users.RemoveMembers: error fetching remaining admins", rz.Err(err))
		return twirp.InternalError(ErrorRemovingMembersMsg)
	}
	if remainingAdmins != 0 {
		return twirp.NewError(twirp.PermissionDenied, "At least one administrator should remain in group.")
	}

	return nil
}

// validateRemoveMembers Checks that user is member of group and he has administrator role
func validateRemoveMembers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, group.ID, user.ID)
	if err != nil {
		logger.Error("groups.RemoveMembers: fetching group membership", rz.Err(err),
			rz.String("group_id", group.ID), rz.String("user_id", user.ID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}

	if memberhsip.Role != RoleAdministrator {
		return twirp.NewError(twirp.PermissionDenied, "Administrator role is required to remove users.")
	}

	return nil
}
