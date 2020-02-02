package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func RemoveMembers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) error {
	logger := rz.FromCtx(ctx)
	var err error
	var remainingAdmins int

	if err = CheckUserIsGroupAdmin(ctx, tx, user.ID, group.ID); err != nil {
		return err
	}

	// delete memberships
	queryDeleteMemberships := `DELETE FROM groups_members
		INNER JOIN users ON users.id = groups_members.user_id
		WHERE users.username IN ($1)`
	_, err = tx.Exec(queryDeleteMemberships, usernames)
	if err != nil {
		logger.Error("groups.RemoveMembers: removing members", rz.Err(err))
		return NewError(ErrorRemovingMembersFromGroup)
	}

	queryRemainingAdmins := `SELECT COUNT(*) FROM groups_members
		WHERE group_id = $1 AND role = $2`
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, RoleAdministrator)
	if err != nil {
		logger.Error("users.RemoveMembers: error fetching remaining admins", rz.Err(err))
		return NewError(ErrorRemovingMembersFromGroup)
	}
	if remainingAdmins != 0 {
		return NewError(ErrorAtLeastOneAdministratorShouldRemainsInGroup)
	}

	return nil
}
