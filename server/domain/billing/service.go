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
	CreatePlan(ctx context.Context, params CreatePlanParams) (Plan, error)
	DeletePlan(ctx context.Context, planID uuid.UUID) error
	RemovePaymentMethod(ctx context.Context, paymentMethodID uuid.UUID) error
	UpdatePlan(ctx context.Context, params UpdatePlanParams) (Plan, error)
	ChangeSubscription(ctx context.Context, params ChangeSubscriptionParams) (Customer, Plan, error)

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

type CreatePlanParams struct {
	Name        string
	StripeID    string
	Description string
	Product     string
	Storage     int64
	IsPublic    bool
}

type UpdatePlanParams struct {
	ID       uuid.UUID
	Name     string
	Product  string
	StripeID string
	// HTML description
	Description string
	IsPublic    bool
	Storage     int64
}

type ChangeSubscriptionParams struct {
	UserID  *uuid.UUID
	GroupID *uuid.UUID
	PlanID  uuid.UUID
}
