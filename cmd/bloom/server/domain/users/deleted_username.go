package users

import (
	"time"
)

// DeletedUsername is used to saves usernames of deleted accounts
type DeletedUsername struct {
	Username  string    `db:"username"`
	DeletedAt time.Time `db:"deleted_at"`
}
