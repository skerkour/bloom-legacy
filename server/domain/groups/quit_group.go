package groups

import (
	"context"

	"github.com/jmoiron/sqlx"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func QuitGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group) error {
	logger := rz.FromCtx(ctx)
	var remainingAdmins int
	var err error

	if err = CheckUserIsGroupMember(ctx, tx, user.ID, group.ID); err != nil {
		return err
	}

	// delete membership
	queryDeleteMembership := "DELETE FROM groups_members WHERE user_id = $1 AND group_id = $2"
	_, err = tx.Exec(queryDeleteMembership, user.ID, group.ID)
	if err != nil {
		logger.Error("groups.QuitGroup: removing members", rz.Err(err))
		return NewError(ErrorQuittingGroup)
	}

	queryRemainingAdmins := "SELECT COUNT(*) FROM groups_members WHERE group_id = $1 AND role = $2"
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, RoleAdministrator)
	if err != nil {
		logger.Error("groups.QuitGroup: error fetching remaining admins", rz.Err(err))
		return NewError(ErrorQuittingGroup)
	}
	if remainingAdmins != 0 {
		return NewError(ErrorAtLeastOneAdministratorShouldRemainsInGroup)
	}

	return nil
}
