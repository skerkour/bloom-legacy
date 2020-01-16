package apictx

import (
	accounts "gitlab.com/bloom42/bloom/server/bloom/accounts/models"
)

type Context struct {
	AuthenticatedAccount *accounts.Account `json:"authenticated_account"`
	Session              *accounts.Session `json:"session"`
	AuthenticatedService *string           `json:"authenticated_service"`
	RequestID            string            `json:"request_id"`
	IP                   string            `json:"ip"`
	UserAgent            string            `json:"user_agent"`
}

// key to use when setting the Context
type key struct{}

// Key is the key that holds the unique Context in a request context.
var Key key = key{}
