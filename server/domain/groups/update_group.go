package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"github.com/twitchtv/twirp"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdateGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group *Group, name, description string) twirp.Error {
	logger := rz.FromCtx(ctx)
	var err error

	if name == "" { // default value of proto3 is empty
		name = group.Name
	}
	if description == "" { // default value of proto3 is empty
		description = group.Description
	}

	if twerr := validateUpdateGroup(ctx, tx, user, *group, name, description); err != nil {
		return twerr
	}

	group.UpdatedAt = time.Now().UTC()
	group.Name = name
	group.Description = description
	queryUpdateGroup := `UPDATE groups
		SET updated_at = $1, name = $2, description = $3
		WHERE id = $4`
	_, err = tx.Exec(queryUpdateGroup, group.UpdatedAt, group.Name, group.Description, group.ID)
	if err != nil {
		logger.Error("groups.UpdateGroup: updating group", rz.Err(err))
		return twirp.InternalError(ErrorUpdatingGroupMsg)
	}

	return nil
}

// validateUpdateGroup Checks that user is member of group and he has administrator role
func validateUpdateGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group Group, name, description string) twirp.Error {
	var memberhsip Membership
	var err error
	logger := rz.FromCtx(ctx)

	queryGetMembership := "SELECT * FROM groups_members WHERE group_id = $1 AND user_id = $2"
	err = tx.Get(&memberhsip, queryGetMembership, group.ID, user.ID)
	if err != nil {
		logger.Error("groups.UpdateGroup: fetching group membership", rz.Err(err),
			rz.String("group_id", group.ID), rz.String("user_id", user.ID))
		return twirp.NewError(twirp.NotFound, "Group not found.")
	}

	if memberhsip.Role != RoleAdministrator {
		return twirp.NewError(twirp.PermissionDenied, "Administrator role is required to update group.")
	}

	if err = validator.GroupName(name); err != nil {
		return twirp.InvalidArgumentError("name", err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return twirp.InvalidArgumentError("description", err.Error())
	}

	return nil
}
