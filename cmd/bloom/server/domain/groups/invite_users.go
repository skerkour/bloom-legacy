package groups

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type InviteUsersParams struct {
	GroupID   uuid.UUID
	Usernames []string
}

func InviteUsers(ctx context.Context, actor *users.User, params InviteUsersParams) (retGroup *Group, err error) {
	logger := rz.FromCtx(ctx)
	inviteesIds := []string{}
	var group Group

	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("groups.InviteUsers: Starting transaction", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	queryGetGroup := "SELECT * FROM groups WHERE id = $1"
	err = tx.Get(&group, queryGetGroup, params.GroupID)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUsers: fetching group", rz.Err(err),
			rz.String("group.id", params.GroupID.String()))
		err = NewError(ErrorGroupNotFound)
		return
	}

	queryStr := "SELECT id FROM users WHERE username IN ($1)"
	query, args, err := sqlx.In(queryStr, params.Usernames)
	query = tx.Rebind(query)
	err = tx.Select(&inviteesIds, query, args...)
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUsers: error fetching invitees ids", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}
	if len(inviteesIds) != len(params.Usernames) {
		tx.Rollback()
		err = NewError(ErrorUsernamesNotFound)
		return
	}

	err = validateInviteUsers(ctx, tx, actor, group, inviteesIds)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("groups.InviteUsers: Committing transaction", rz.Err(err))
		err = NewError(ErrorInvitingUsers)
		return
	}

	retGroup = &group
	// TODO: create invitations
	return
}

func validateInviteUsers(ctx context.Context, tx *sqlx.Tx, inviter *users.User, group Group, inviteesIds []string) error {
	logger := rz.FromCtx(ctx)
	var alreadyInUsers int
	var err error

	// check that inviter inviting is admin
	if err = CheckUserIsGroupAdmin(ctx, tx, inviter.ID, group.ID); err != nil {
		return err
	}

	//  check that user is not already in group or awaiting invitations
	queryStr := `SELECT COUNT(*)
		FROM groups_members, groups_invitations
		WHERE (groups_members.group_id = $1 AND groups_members.user_id IN ($2))
			OR (groups_invitations.invitee_id IN ($2))`

	query, args, err := sqlx.In(queryStr, group.ID, inviteesIds)
	query = tx.Rebind(query)
	err = tx.Get(&alreadyInUsers, query, args...)
	if err != nil {
		logger.Error("groups.InviteUsers: error fetching users already in group or invitations", rz.Err(err))
		return NewError(ErrorInvitingUsers)
	}
	if alreadyInUsers != 0 {
		return NewError(ErrorUserAlreadyInGroup)
	}

	return nil
}
