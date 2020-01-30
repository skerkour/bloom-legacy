package users

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/common/validator"
	"gitlab.com/bloom42/libs/crypto42-go/password/argon2id"
	"gitlab.com/bloom42/libs/rz-go"
)

func CreateUser(ctx context.Context, tx *sqlx.Tx, pendingUser PendingUser, username string, authKey []byte) (User, error) {
	logger := rz.FromCtx(ctx)
	var err error
	var ret User
	var existingUser int

	// validate params
	if err = validator.UserUsername(username); err != nil {
		return ret, NewErrorMessage(ErrorInvalidArgument, err.Error()) //twirp.InvalidArgumentError("username", err.Error())
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM users WHERE email = $1"
	err = tx.Get(&existingUser, queryCountExistingEmails, pendingUser.Email)
	if err != nil {
		logger.Error("users.CreateUser: error fetching existing emails counts", rz.Err(err))
		return ret, NewError(ErrorEmailAlreadyExists)
	}
	if existingUser != 0 {
		return ret, NewError(ErrorEmailAlreadyExists)
		// twirp.InvalidArgumentError("email", fmt.Sprintf("user with email: '%s' already exists", pendingUser.Email))
	}

	// verify that username isn't already in use
	existingUser = 0
	queryCountExistingUsername := "SELECT COUNT(*) FROM users WHERE username = $1"
	err = tx.Get(&existingUser, queryCountExistingUsername, username)
	if err != nil {
		logger.Error("users.CreateUser: error fetching existing username counts", rz.Err(err))
		return ret, NewError(ErrorUsernameAlreadyExists)
	}
	if existingUser != 0 {
		return ret, NewError(ErrorUsernameAlreadyExists)
	}

	// verify that username was not used by a deleted user
	existingUser = 0
	queryCountDeletedUsername := "SELECT COUNT(*) FROM deleted_usernames WHERE username = $1"
	err = tx.Get(&existingUser, queryCountDeletedUsername, username)
	if err != nil {
		logger.Error("users.CreateUser: error fetching deleted username counts", rz.Err(err))
		return ret, NewError(ErrorUsernameAlreadyExists)
	}
	if existingUser != 0 {
		return ret, NewError(ErrorUsernameAlreadyExists)
	}

	now := time.Now().UTC()
	newUuid := uuid.New()
	// TODO: update params
	authKeyHash, err := argon2id.HashPassword(authKey, argon2id.DefaultHashPasswordParams)
	if err != nil {
		logger.Error("users.CreateUser: hashing auth key", rz.Err(err))
		return ret, NewError(ErrorCompletingRegistration)
	}

	ret = User{
		ID:          newUuid.String(),
		Username:    username,
		Email:       pendingUser.Email,
		CreatedAt:   now,
		UpdatedAt:   now,
		DisplayName: pendingUser.DisplayName,
		AuthKeyHash: authKeyHash,
	}

	queryCreateUser := `INSERT INTO users
		(id, created_at, updated_at, username, display_name, bio, email, first_name, last_name, is_admin, auth_key_hash)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`
	_, err = tx.Exec(queryCreateUser, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Username, ret.DisplayName, "", ret.Email, "", "", false, ret.AuthKeyHash)
	if err != nil {
		logger.Error("users.CreateUser: inserting new user", rz.Err(err))
		return ret, NewError(ErrorCompletingRegistration)
	}

	return ret, nil
}
