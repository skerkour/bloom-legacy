package groups

import (
	"time"

	"gitlab.com/bloom42/bloom/server/domain/users"
)

type Member struct {
	users.User
	Role     string    `db:"role"`
	JoinedAt time.Time `db:"joined_at"`
}
