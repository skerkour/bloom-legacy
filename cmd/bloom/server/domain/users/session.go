package users

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type Session struct {
	ID         uuid.UUID `json:"id" db:"id"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
	UpdatedAt  time.Time `json:"updated_at" db:"updated_at"`
	Hash       []byte    `json:"hash" db:"hash"`
	Salt       []byte    `json:"salt" db:"salt"`
	DeviceOS   string    `json:"device_os" db:"device_os"`
	DeviceType string    `json:"device_type" db:"device_type"`
	UserID     uuid.UUID `json:"user_id" db:"user_id"`
}

type SessionDevice struct {
	OS   string
	Type string
}

func FindAllSessionsByUserId(ctx context.Context, userId uuid.UUID) ([]Session, error) {
	ret := []Session{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM sessions WHERE user_id = $1"
	err = db.DB.Select(&ret, queryFind, userId)

	if err != nil {
		logger.Error("users.FindAllSessionsByUserId: finding sessions", rz.Err(err),
			rz.String("user.id", userId.String()))
		return ret, NewError(-1)
	}

	return ret, err
}
