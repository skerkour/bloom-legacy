package groups

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func UpdateGroup(ctx context.Context, tx *sqlx.Tx, user users.User, group *Group, name, description *string) error {
	logger := rz.FromCtx(ctx)
	var err error
	var newName string
	var newDescription string

	if name == nil {
		newName = group.Name
	} else {
		newName = *name
	}

	if description == nil {
		newDescription = group.Description
	} else {
		newDescription = *description
	}

	if err = validateUpdateGroup(ctx, tx, user.ID, group.ID, newName, newDescription); err != nil {
		return err
	}

	group.UpdatedAt = time.Now().UTC()
	group.Name = newName
	group.Description = newDescription
	queryUpdateGroup := `UPDATE groups
		SET updated_at = $1, name = $2, description = $3
		WHERE id = $4`
	_, err = tx.Exec(queryUpdateGroup, group.UpdatedAt, group.Name, group.Description, group.ID)
	if err != nil {
		logger.Error("groups.UpdateGroup: updating group", rz.Err(err))
		return NewError(ErrorUpdatingGroup)
	}

	return nil
}

// validateUpdateGroup Checks that user is member of group and he has administrator role
func validateUpdateGroup(ctx context.Context, tx *sqlx.Tx, userID, groupID, name, description string) error {
	var err error

	if err = CheckUserIsGroupAdmin(ctx, tx, userID, groupID); err != nil {
		return err
	}

	if err = validator.GroupName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	return nil
}
