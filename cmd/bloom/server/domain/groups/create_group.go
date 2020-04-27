package groups

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

// CreateGroupParams are parameters required to create a group
type CreateGroupParams struct {
	Name          string
	Description   string
	UsersToInvite []string
}

// CreateGroup creates a group
func CreateGroup(ctx context.Context, tx *sqlx.Tx, actor *users.User, params CreateGroupParams) (ret *Group, err error) {
	logger := rz.FromCtx(ctx)

	// clean and validate params
	params.Name = strings.TrimSpace(params.Name)
	params.Description = strings.TrimSpace(params.Description)
	err = validateCreateGroup(params.Name, params.Description)
	if err != nil {
		return ret, err
	}

	now := time.Now().UTC()
	ret = &Group{
		ID:          uuid.New(),
		CreatedAt:   now,
		UpdatedAt:   now,
		Name:        params.Name,
		Description: params.Description,
	}

	// create group
	queryCreateGroup := `INSERT INTO groups
		(id, created_at, updated_at, name, description)
		VALUES ($1, $2, $3, $4, $5)`
	_, err = tx.Exec(queryCreateGroup, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description)
	if err != nil {
		logger.Error("groups.CreateGroup: inserting new group", rz.Err(err))
		return ret, NewError(ErrorCreatingGroup)
	}

	// admin creator to group
	queryAddAdminToGroup := `INSERT INTO groups_members
	(user_id, group_id, role, joined_at, inviter_id)
	VALUES ($1, $2, $3, $4, $1)`
	_, err = tx.Exec(queryAddAdminToGroup, actor.ID, ret.ID, RoleAdministrator, now)
	if err != nil {
		logger.Error("groups.CreateGroup: inserting admin in new group", rz.Err(err))
		return ret, NewError(ErrorCreatingGroup)
	}

	return ret, nil
}

func validateCreateGroup(name, description string) error {
	var err error

	if err = ValidateGroupName(name); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	if err = ValidateGroupDescription(description); err != nil {
		return NewErrorMessage(ErrorInvalidArgument, err.Error())
	}

	return nil
}
