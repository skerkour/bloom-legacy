package groups

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/server/domain/users"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

// CreateGroupParams are parameters required to create a group
type CreateGroupParams struct {
	Name               string
	Description        string
	EncryptedMasterKey []byte
	MasterKeyNonce     []byte
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
		State:       0,
	}

	// create group
	queryCreateGroup := `INSERT INTO groups
		(id, created_at, updated_at, name, description, state)
		VALUES ($1, $2, $3, $4, $5, $6)`
	_, err = tx.Exec(queryCreateGroup, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Name, ret.Description, ret.State)
	if err != nil {
		logger.Error("groups.CreateGroup: inserting new group", rz.Err(err))
		return ret, NewError(ErrorCreatingGroup)
	}

	// admin creator to group
	queryAddAdminToGroup := `INSERT INTO groups_members
	(user_id, inviter_id, group_id, joined_at, role, encrypted_master_key, master_key_nonce)
	VALUES ($1, $1, $2, $3, $4, $5, $6)`
	_, err = tx.Exec(queryAddAdminToGroup, actor.ID, ret.ID, now, consts.GROUP_ROLE_ADMINISTRATOR,
		params.EncryptedMasterKey, params.MasterKeyNonce)
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
