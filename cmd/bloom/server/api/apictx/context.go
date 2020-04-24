package apictx

import (
	"gitlab.com/bloom42/bloom/cmd/bloom/server/domain/users"
	"gitlab.com/bloom42/lily/uuid"
)

type Context struct {
	AuthenticatedUser    *users.User    `json:"authenticated_user"`
	Session              *users.Session `json:"session"`
	AuthenticatedService *string        `json:"authenticated_service"`
	RequestID            uuid.UUID      `json:"request_id"`
	IP                   string         `json:"ip"`
	UserAgent            string         `json:"user_agent"`
}

// key to use when setting the Context
type key struct{}

// Key is the key that holds the unique Context in a request context.
var Key key = key{}
