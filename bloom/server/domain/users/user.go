package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
)

type User struct {
	ID                  string     `db:"id"`
	CreatedAt           time.Time  `db:"created_at"`
	UpdatedAt           time.Time  `db:"updated_at"`
	AvatardID           *string    `db:"avatar_id"`
	Email               string     `db:"email"`
	DisplayName         string     `db:"display_name"`
	Username            string     `db:"username"`
	AuthKeyHash         string     `db:"auth_key_hash"`
	Bio                 string     `db:"bio"`
	FirstName           string     `db:"first_name"`
	LastName            string     `db:"last_name"`
	IsAdmin             bool       `db:"is_admin"`
	DisabledAt          *time.Time `db:"disabled_at"`
	PublicKey           []byte     `db:"public_key"`
	EncryptedPrivateKey []byte     `db:"encrypted_private_key"`
}

func FindUserByUsername(ctx context.Context, tx *sqlx.Tx, username string) (*User, error) {
	ret := &User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users WHERE username = $1"
	err = tx.Get(ret, queryFind, username)
	if err != nil {
		logger.Error("users.FindUserByUsername: finding user", rz.Err(err),
			rz.String("username", username))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
}

func FindUserByUsernameNoTx(ctx context.Context, username string) (*User, error) {
	ret := &User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users WHERE username = $1"
	err = db.DB.Get(ret, queryFind, username)
	if err != nil {
		logger.Error("users.FindUserByUsernameNoTx: finding user", rz.Err(err),
			rz.String("username", username))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
}
