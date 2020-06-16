package driver

import (
	"context"

	"gitlab.com/bloom42/gobox/email"
)

// Mailer is the interface defining a service which sends emails
type Mailer interface {
	Send(ctx context.Context, email email.Email) error
}
