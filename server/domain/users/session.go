package users

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type Session struct {
	ID         string    `json:"id" db:"id"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
	UpdatedAt  time.Time `json:"updated_at" db:"updated_at"`
	TokenHash  string    `json:"token_hash" db:"token_hash"`
	DeviceOS   string    `json:"device_os" db:"device_os"`
	DeviceType string    `json:"device_type" db:"device_type"`
	UserID     string    `json:"user_id" db:"user_id"`
}

type SessionDevice struct {
	OS   string
	Type string
}

func FindSessionById(ctx context.Context, tx *sqlx.Tx, id string) (*Session, error) {
	ret := &Session{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM sessions WHERE id = $1"
	err = tx.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("users.FindSessionById: finding session", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorSessionNotFound)
	}

	return ret, err
}

func FindSessionByIdNoTx(ctx context.Context, id string) (*Session, error) {
	ret := &Session{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM sessions WHERE id = $1"
	err = db.DB.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("users.FindSessionById: finding session", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorSessionNotFound)
	}

	return ret, err
}

func FindAllSessionsByUserId(ctx context.Context, userId string) ([]Session, error) {
	ret := []Session{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM sessions WHERE user_id = $1"
	err = db.DB.Select(&ret, queryFind, userId)

	if err != nil {
		logger.Error("users.FindAllSessionsByUserId: finding sessions", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(-1)
	}

	return ret, err
}
