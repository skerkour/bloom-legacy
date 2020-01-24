package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func DeleteGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error

	if twerr := validateDeleteGroup(ctx, tx, user, group); err != nil {
		return twerr
	}

	// delete group
	queryDeleteGroup := "DELETE FROM groups WHERE id = $1"
	_, err = tx.Exec(queryDeleteGroup, group.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleting group", rz.Err(err))
		return twirp.InternalError(ErrorDeleteGroupMsg)
	}

	// delete members
	queryDeleteGroupMembers := "DELETE FROM groups_members WHERE group_id = $1"
	_, err = tx.Exec(queryDeleteGroupMembers, group.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: deleteing members", rz.Err(err))
		return twirp.InternalError(ErrorDeleteGroupMsg)
	}

	return nil
}

func validateDeleteGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, group.ID, user.ID)
	if err != nil {
		logger.Error("groups.DeleteGroup: fetching group membership", rz.Err(err),
			rz.String("group_id", group.ID), rz.String("user_id", user.ID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}

	if memberhsip.Role != RoleAdministrator {
		return twirp.NewError(twirp.PermissionDenied, "Administrator role is required to delete group.")
	}

	return nil
}
