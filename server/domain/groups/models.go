package groups

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type Group struct {
	ID          string    `json:"id" db:"id"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time `json:"updated_at" db:"updated_at"`
	Name        string    `json:"name" db:"name"`
	Description string    `json:"description" db:"description"`
	AvatardID   *string   `json:"avatar_id" db:"avatar_id"`
}

type Membership struct {
	JoinedAt time.Time `json:"joined_at" db:"joined_at"`
	GroupID  string    `json:"group_id" db:"group_id"`
	UserID   string    `json:"user_id" db:"user_id"`
	Role     string    `json:"role" db:"role"`
}

type Invitation struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	GroupID   string `json:"group_id" db:"group_id"`
	InviteeID string `json:"invitee_id" db:"invitee_id"`
}

func FindGroupById(ctx context.Context, id string) (*Group, error) {
	ret := &Group{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM groups WHERE id = $1"
	err = db.DB.Get(ret, queryFind, id)
	if err != nil {
		logger.Error("groups.FindGroupById: finding user", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorGroupNotFound)
	}

	return ret, err
}
