package groups

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreateGroup(ctx context.Context, tx *sqlx.Tx, admin users.User, name, description string) (Group, error) {
	logger := rz.FromCtx(ctx)
	ret := Group{}
	var err error

	err = validateCreateGroup(name, description)
	if err != nil {
		return ret, err
	}

	now := time.Now().UTC()
	newUuid := uuid.New()

	ret = Group{
		ID:          newUuid.String(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Name:        name,
		Description: description,
	}

	// create group
	queryCreateGroup := `INSERT INTO groups
		(id, created_at, updated_at, name, description)
		VALUES ($1, $2, $3, $4, $5)`
	_, err = tx.Exec(queryCreateGroup, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description)
	if err != nil {
		logger.Error("inserting new group", rz.Err(err))
		return ret, NewError(ErrorCreatingGroup)
	}

	// admin creator to group
	queryAddAdminToGroup := `INSERT INTO groups_members
	(user_id, group_id, role, joined_at)
	VALUES ($1, $2, $3, $4)`
	_, err = tx.Exec(queryAddAdminToGroup, admin.ID, ret.ID, RoleAdministrator, now)
	if err != nil {
		logger.Error("inserting admin in new group", rz.Err(err))
		return ret, NewError(ErrorCreatingGroup)
	}

	return ret, nil
}

func validateCreateGroup(name, description string) error {
	var err error

	if err = validator.GroupName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	return nil
}
