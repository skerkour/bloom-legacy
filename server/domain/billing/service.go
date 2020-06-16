package billing

import (
	"context"

	"github.com/stripe/stripe-go"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/gobox/uuid"
)

// Service is the application service for the billing subdomain
type Service interface {
	// Commands
	CreateCustomer(ctx context.Context, db db.Queryer, user users.User, groupID *uuid.UUID) (Customer, error)
	CreateOrUpdateInvoice(ctx context.Context, invoice stripe.Invoice) error
	PaymentFailed(ctx context.Context, invoice stripe.Invoice) error
	// Queries
}
