package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func QuitGroup(ctx context.Context, actor *users.User, groupID uuid.UUID) (err error) {
	logger := rz.FromCtx(ctx)
	var remainingAdmins int

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.QuitGroup: Starting transaction", rz.Err(err))
		err = NewError(ErrorQuittingGroup)
		return
	}

	group, err := FindGroupById(ctx, tx, groupID, false)
	if err != nil {
		tx.Rollback()
		err = NewError(ErrorGroupNotFound)
		return
	}

	if err = CheckUserIsGroupMember(ctx, tx, actor.ID, groupID); err != nil {
		tx.Rollback()
		return err
	}

	// delete membership
	queryDeleteMembership := "DELETE FROM groups_members WHERE user_id = $1 AND group_id = $2"
	_, err = tx.Exec(queryDeleteMembership, actor.ID, group.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.QuitGroup: removing member", rz.Err(err))
		err = NewError(ErrorQuittingGroup)
		return
	}

	queryRemainingAdmins := "SELECT COUNT(*) FROM groups_members WHERE group_id = $1 AND role = $2"
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, consts.GROUP_ROLE_ADMINISTRATOR)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.QuitGroup: error fetching remaining admins", rz.Err(err))
		err = NewError(ErrorQuittingGroup)
		return
	}
	if remainingAdmins == 0 {
		tx.Rollback()
		err = NewError(ErrorAtLeastOneAdministratorShouldRemainsInGroup)
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.QuitGroup: Committing transaction", rz.Err(err))
		err = NewError(ErrorQuittingGroup)
		return
	}

	return
}
