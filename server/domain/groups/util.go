package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/libs/rz-go"
)

// checkUserIsGroupAdmin Checks that user is member of group and he has administrator role
func CheckUserIsGroupAdmin(ctx context.Context, tx *sqlx.Tx, userID, groupID string) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}

	if memberhsip.Role != RoleAdministrator {
		return twirp.NewError(twirp.PermissionDenied, "Administrator role is required to delete group.")
	}

	return nil
}

// checkUserIsGroupMember Checks that user is member of group
func checkUserIsGroupMember(ctx context.Context, tx *sqlx.Tx, userID, groupID string) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}

	return nil
}
