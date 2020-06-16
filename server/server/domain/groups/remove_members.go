package groups

import (
	"context"
	"strings"

	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/server/db"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

type RemoveMembersParams struct {
	GroupID   uuid.UUID
	Usernames []string
}

func RemoveMembers(ctx context.Context, actor *users.User, params RemoveMembersParams) (ret *Group, err error) {
	logger := rz.FromCtx(ctx)
	var remainingAdmins int
	var group Group

	if len(params.Usernames) != 1 {
		err = NewError(ErrorRemovingMembersFromGroup)
		return
	}

	username := strings.ToLower(params.Usernames[0])
	username = strings.TrimSpace(username)

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.RemoveMembers: Starting transaction", rz.Err(err))
		err = NewError(ErrorRemovingMembersFromGroup)
		return
	}

	err = CheckUserIsGroupAdmin(ctx, tx, actor.ID, params.GroupID)
	if err != nil {
		tx.Rollback()
		return
	}

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, params.GroupID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.RemoveMembers: fetching group", rz.Err(err),
			rz.String("group.id", params.GroupID.String()))
		err = NewError(ErrorGroupNotFound)
		return
	}

	user, err := users.FindUserByUsername(ctx, tx, username)
	if err != nil {
		tx.Rollback()
		return
	}

	// delete memberships
	queryDeleteMembership := `DELETE FROM groups_members
		WHERE groups_members.user_id = $1`
	_, err = tx.Exec(queryDeleteMembership, user.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.RemoveMembers: removing members", rz.Err(err))
		err = NewError(ErrorRemovingMembersFromGroup)
		return
	}

	// // delete memberships
	// queryStr := `DELETE FROM groups_members
	// 	INNER JOIN users ON users.id = groups_members.user_id
	// 	WHERE users.username IN ($1)`
	// query, args, err := sqlx.In(queryStr, params.Usernames)
	// query = tx.Rebind(query)
	// _, err = tx.Exec(query, args...)
	// if err != nil {
	// 	tx.Rollback()
	// 	logger.Error("groups.RemoveMembers: removing members", rz.Err(err))
	// 	err = NewError(ErrorRemovingMembersFromGroup)
	// 	return
	// }

	queryRemainingAdmins := `SELECT COUNT(*) FROM groups_members
		WHERE group_id = $1 AND role = $2`
	err = tx.Get(&remainingAdmins, queryRemainingAdmins, group.ID, consts.GROUP_ROLE_ADMINISTRATOR)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.RemoveMembers: error fetching remaining admins", rz.Err(err))
		err = NewError(ErrorRemovingMembersFromGroup)
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
		logger.Error("groups.RemoveMembers: Committing transaction", rz.Err(err))
		err = NewError(ErrorRemovingMembersFromGroup)
	}

	ret = &group
	return
}
