package groups

import (
	"context"
	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func QuitGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	logger := rz.FromCtx(ctx)
	var remainingAdmins int
	var err error

	if twerr := validateQuitGroup(ctx, tx, user, group); err != nil {
		return twerr
	}

	// delete membership
	queryDeleteMembership := "DELETE FROM groups_members WHERE user_id = $1 AND group_id = $2"
	_, err = tx.Exec(queryDeleteMembership, user.ID, group.ID)
	if err != nil {
		logger.Error("groups.QuitGroup: removing members", rz.Err(err))
		return twirp.InternalError(ErrorQuittingGroupMsg)
	}

	queryRemainingAdmins := "SELECT COUNT(*) FROM groups_members WHERE group_id = $1 AND role = $2"
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, RoleAdministrator)
	if err != nil {
		logger.Error("users.QuitGroup: error fetching remaining admins", rz.Err(err))
		return twirp.InternalError(ErrorQuittingGroupMsg)
	}
	if remainingAdmins != 0 {
		return twirp.NewError(twirp.PermissionDenied, "At least one administrator should remain in group.")
	}
	return nil
}

// validateQuitGroup Checks that user is member of group
func validateQuitGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, group.ID, user.ID)
	if err != nil {
		logger.Error("groups.QuitGroup: fetching group membership", rz.Err(err),
			rz.String("group_id", group.ID), rz.String("user_id", user.ID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}
	return nil
}
