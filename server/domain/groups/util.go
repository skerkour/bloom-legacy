package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

// checkUserIsGroupAdmin Checks that user is member of group and he has administrator role
func CheckUserIsGroupAdmin(ctx context.Context, tx *sqlx.Tx, userID, groupID string) error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return NewError(ErrorGroupNotFound)
	}

	if memberhsip.Role != RoleAdministrator {
		return NewErrorMessage(ErrorPermissionDenied, "Administrator role is required.")
	}

	return nil
}

// checkUserIsGroupAdmin Checks that user is member of group and he has administrator role
func CheckUserIsGroupAdminNoTx(ctx context.Context, userID, groupID string) error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = db.DB.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return NewError(ErrorGroupNotFound)
	}

	if memberhsip.Role != RoleAdministrator {
		return NewErrorMessage(ErrorPermissionDenied, "Administrator role is required.")
	}

	return nil
}

// CheckUserIsGroupMember Checks that user is member of group
func CheckUserIsGroupMember(ctx context.Context, tx *sqlx.Tx, userID, groupID string) error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return NewError(ErrorGroupNotFound)
	}

	return nil
}

// CheckUserIsGroupMemberNoTx Checks that user is member of group
func CheckUserIsGroupMemberNoTx(ctx context.Context, userID, groupID string) error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = db.DB.Get(&memberhsip, queryGetMembership, groupID, userID)
	if err != nil {
		logger.Error("groups.checkUserIsGroupAdmin: fetching group membership", rz.Err(err),
			rz.String("group_id", groupID), rz.String("user_id", userID))
		return NewError(ErrorGroupNotFound)
	}

	return nil
}
