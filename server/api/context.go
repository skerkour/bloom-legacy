package api

import (
	"gitlab.com/bloom42/bloom/server/bloom/accounts"
)

type Context struct {
	AuthenticatedAccount *accounts.Account `json:"authenticated_account"`
	Session              *accounts.Session `json:"session"`
	AuthenticatedService *string           `json:"authenticated_service"`
	RequestID            string            `json:"request_id"`
	IP                   string            `json:"ip"`
	UserAgent            string            `json:"user_agent"`
}

// Key to use when setting the Context
type ctxKeytx struct{}

// ctxKeytx is the key that holds the unique Context in a request context.
var CtxKey ctxKeytx = ctxKeytx{}
