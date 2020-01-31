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

func CreateGroup(ctx context.Context, tx *sqlx.Tx, admin users.User, name, description string) (Group, twirp.Error) {
	logger := rz.FromCtx(ctx)
	ret := Group{}
	var err error

	twerr := validateCreateGroup(name, description)
	if twerr != nil {
		return ret, twerr
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
		logger.Error("groups.CreateGroup: inserting new group", rz.Err(err))
		return ret, twirp.InternalError(ErrorCreateGroupMsg)
	}

	// admin creator to group
	queryAddAdminToGroup := `INSERT INTO groups_members
	(user_id, group_id, role)
	VALUES ($1, $2, $3)`
	_, err = tx.Exec(queryAddAdminToGroup, admin.ID, ret.ID, RoleAdministrator)
	if err != nil {
		logger.Error("groups.CreateGroup: inserting aadmin in new group", rz.Err(err))
		return ret, twirp.InternalError(ErrorCreateGroupMsg)
	}

	return ret, nil
}

func validateCreateGroup(name, description string) twirp.Error {
	var err error

	if err = validator.GroupName(name); err != nil {
		return twirp.InvalidArgumentError("name", err.Error())
	}

	if err = validator.GroupDescription(description); err != nil {
		return twirp.InvalidArgumentError("description", err.Error())
	}

	return nil
}
