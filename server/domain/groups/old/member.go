package groups

import (
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

// Member is used to list group members
type Member struct {
	users.User
	Role     string    `db:"role"`
	JoinedAt time.Time `db:"joined_at"`
}
