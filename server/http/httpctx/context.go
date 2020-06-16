package httpctx

import (
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

// Context is used to carry data during a request' lifecycle
type Context struct {
	AuthenticatedUser *users.User
	Session           *users.Session
	RequestID         uuid.UUID
	IP                string
	UserAgent         string
}

// key type to use when setting the Context
type key struct{}

// Key is the key that holds the unique Context in a request context.
var Key key = key{}
