package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type User struct {
	ID          string     `json:"id" db:"id"`
	CreatedAt   time.Time  `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time  `json:"updated_at" db:"updated_at"`
	AvatardID   *string    `json:"avatar_id" db:"avatar_id"`
	Email       string     `json:"email" db:"email"`
	DisplayName string     `json:"display_name" db:"display_name"`
	Username    string     `json:"username" db:"username"`
	AuthKeyHash string     `json:"-" db:"auth_key_hash"`
	Bio         string     `json:"bio" db:"bio"`
	FirstName   string     `json:"first_name" db:"first_name"`
	LastName    string     `json:"last_name" db:"last_name"`
	IsAdmin     bool       `json:"is_admin" db:"is_admin"`
	DisabledAt  *time.Time `json:"disabled_at" db:"disabled_at"`
}

func FindUserById(ctx context.Context, tx *sqlx.Tx, id string) (*User, error) {
	ret := &User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users WHERE id = $1"
	err = tx.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("users.FindUserById: finding user", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
}

func FindUserByIdNoTx(ctx context.Context, id string) (*User, error) {
	ret := &User{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM users WHERE id = $1"
	err = db.DB.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("users.FindUserById: finding user", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorUserNotFound)
	}

	return ret, err
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
