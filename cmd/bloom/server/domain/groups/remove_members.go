package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
)

func RemoveMembers(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, usernames []string) error {
	logger := rz.FromCtx(ctx)
	var err error
	var remainingAdmins int

	if err = CheckUserIsGroupAdmin(ctx, tx, user.ID, group.ID); err != nil {
		return err
	}

	// delete memberships
	queryStr := `DELETE FROM groups_members
		INNER JOIN users ON users.id = groups_members.user_id
		WHERE users.username IN ($1)`
	query, args, err := sqlx.In(queryStr, usernames)
	query = tx.Rebind(query)
	_, err = tx.Exec(query, args...)
	if err != nil {
		logger.Error("removing members", rz.Err(err))
		return NewError(ErrorRemovingMembersFromGroup)
	}

	queryRemainingAdmins := `SELECT COUNT(*) FROM groups_members
		WHERE group_id = $1 AND role = $2`
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, RoleAdministrator)
	if err != nil {
		logger.Error("error fetching remaining admins", rz.Err(err))
		return NewError(ErrorRemovingMembersFromGroup)
	}
	if remainingAdmins != 0 {
		return NewError(ErrorAtLeastOneAdministratorShouldRemainsInGroup)
	}

	return nil
}
