package users

import (
	"context"
	"strings"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type createUserParams struct {
	Email               string
	DisplayName         string
	Username            string
	AuthKey             []byte
	PublicKey           []byte
	EncryptedPrivateKey []byte
	PrivateKeyNonce     []byte
	EncryptedMasterKey  []byte
	MasterKeyNonce      []byte
}

func createUser(ctx context.Context, tx *sqlx.Tx, params createUserParams) (ret *User, err error) {
	logger := rz.FromCtx(ctx)
	var existingUser int

	// validate params
	params.Username = strings.TrimSpace(params.Username)
	err = ValidateUsername(params.Username)
	if err != nil {
		err = NewErrorMessage(ErrorInvalidArgument, err.Error())
		return
	}

	// check if email does not already exist
	queryCountExistingEmails := "SELECT COUNT(*) FROM users WHERE email = $1"
	err = tx.Get(&existingUser, queryCountExistingEmails, params.Email)
	if err != nil {
		logger.Error("users.CreateUser: error fetching existing emails counts", rz.Err(err))
		err = NewError(ErrorEmailAlreadyExists)
		return
	}
	if existingUser != 0 {
		err = NewError(ErrorEmailAlreadyExists)
		return
		// twirp.InvalidArgumentError("email", fmt.Sprintf("user with email: '%s' already exists", pendingUser.Email))
	}

	// verify that username isn't already in use
	existingUser = 0
	queryCountExistingUsername := "SELECT COUNT(*) FROM users WHERE username = $1"
	err = tx.Get(&existingUser, queryCountExistingUsername, params.Username)
	if err != nil {
		logger.Error("users.CreateUser: error fetching existing username counts", rz.Err(err))
		err = NewError(ErrorUsernameAlreadyExists)
		return
	}
	if existingUser != 0 {
		err = NewError(ErrorUsernameAlreadyExists)
		return
	}

	// verify that username was not used by a deleted user
	existingUser = 0
	queryCountDeletedUsername := "SELECT COUNT(*) FROM deleted_usernames WHERE username = $1"
	err = tx.Get(&existingUser, queryCountDeletedUsername, params.Username)
	if err != nil {
		logger.Error("users.CreateUser: error fetching deleted username counts", rz.Err(err))
		err = NewError(ErrorUsernameAlreadyExists)
		return
	}
	if existingUser != 0 {
		err = NewError(ErrorUsernameAlreadyExists)
		return
	}

	authKeyHash, err := crypto.HashPassword(params.AuthKey, AUTH_KEY_HASH_PARAMS)
	if err != nil {
		logger.Error("users.CreateUser: hashing auth key", rz.Err(err))
		err = NewError(ErrorCompletingRegistration)
		return
	}

	now := time.Now().UTC()
	ret = &User{
		ID:                  uuid.New(),
		CreatedAt:           now,
		UpdatedAt:           now,
		DisabledAt:          nil,
		Username:            params.Username,
		Email:               params.Email,
		DisplayName:         params.DisplayName,
		Bio:                 "",
		FirstName:           "",
		LastName:            "",
		AuthKeyHash:         authKeyHash,
		State:               0,
		IsAdmin:             false,
		PublicKey:           params.PublicKey,
		EncryptedPrivateKey: params.EncryptedPrivateKey,
		PrivateKeyNonce:     params.PrivateKeyNonce,
		EncryptedMasterKey:  params.MasterKeyNonce,
		MasterKeyNonce:      params.MasterKeyNonce,
		TwoFASecret:         nil,
	}

	queryCreateUser := `INSERT INTO users
		(id, created_at, updated_at, username, display_name, bio, email, first_name, last_name,
			is_admin, auth_key_hash, public_key, encrypted_private_key, state, private_key_nonce)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)`
	_, err = tx.Exec(queryCreateUser, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.Username,
		ret.DisplayName, ret.Bio, ret.Email, ret.FirstName, ret.LastName, false, ret.AuthKeyHash, ret.PublicKey,
		ret.EncryptedPrivateKey, ret.State, ret.PrivateKeyNonce)
	if err != nil {
		logger.Error("users.CreateUser: inserting new user", rz.Err(err))
		err = NewError(ErrorCompletingRegistration)
		return
	}

	return ret, nil
}
