package apictx

import (
	"gitlab.com/bloom42/bloom/server/domain/users"
)

type Context struct {
	AuthenticatedUser    *users.User    `json:"authenticated_user"`
	Session              *users.Session `json:"session"`
	AuthenticatedService *string        `json:"authenticated_service"`
	RequestID            string         `json:"request_id"`
	IP                   string         `json:"ip"`
	UserAgent            string         `json:"user_agent"`
}

// key to use when setting the Context
type key struct{}

// Key is the key that holds the unique Context in a request context.
var Key key = key{}
