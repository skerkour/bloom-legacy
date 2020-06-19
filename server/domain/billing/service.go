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
	AddPaymentMethod(ctx context.Context, params AddPaymentMethodParams) (PaymentMethod, error)
	ChangeDefaultPaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) (PaymentMethod, error)
	RemovePaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) error
	UpdatePlan(ctx context.Context, params UpdatePlanParams) (Plan, error)
	ChangeSubscription(ctx context.Context, params ChangeSubscriptionParams) (Customer, Plan, error)
	UpdateCustomerUsedStorage(ctx context.Context, db db.Queryer, customer Customer, storage int64) (Customer, error)
	DetachCustomer(ctx context.Context, db db.Queryer, user users.User, groupID *uuid.UUID) (err error)

	// Queries
	FindAllPlans(ctx context.Context) ([]Plan, error)
	SubscribersCountForPlan(ctx context.Context, planID uuid.UUID) (int64, error)
	FindInvoicesForUser(ctx context.Context, userID uuid.UUID) ([]Invoice, error)
	FindPaymentMethodsForUser(ctx context.Context, userID uuid.UUID) ([]PaymentMethod, error)
	SubscriptionForUser(ctx context.Context, userID uuid.UUID) (Customer, Plan, error)
	SubscriptionForGroup(ctx context.Context, groupID uuid.UUID) (Customer, Plan, error)
	FindInvoicesForGroup(ctx context.Context, groupID uuid.UUID) ([]Invoice, error)
	FindPaymentMethodsForGroup(ctx context.Context, groupID uuid.UUID) ([]PaymentMethod, error)
}

type AddPaymentMethodParams struct {
	StripeID string
	GroupID  *uuid.UUID
}

type UpdatePlanParams struct {
	ID       uuid.UUID
	Name     string
	Product  string
	StripeID string
	// HTML description
	Description string
	Storage     int64
}

type ChangeSubscriptionParams struct {
	UserID  *uuid.UUID
	GroupID *uuid.UUID
	PlanID  uuid.UUID
}
