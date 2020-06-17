package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the billing subdomain
type Repository interface {
	UpdateCustomer(ctx context.Context, db db.Queryer, customer Customer) error
	CreatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod PaymentMethod) error
	UpdatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod PaymentMethod) error
	CreateCustomer(ctx context.Context, db db.Queryer, customer Customer) error
	CreateInvoice(ctx context.Context, db db.Queryer, invoice Invoice) error
	UpdateInvoice(ctx context.Context, db db.Queryer, invoice Invoice) error
	CreatePlan(ctx context.Context, db db.Queryer, plan Plan) error
	DeletePlan(ctx context.Context, db db.Queryer, planID uuid.UUID) error
	DeletePaymentMethod(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) error

	FindCustomerByUserID(ctx context.Context, db db.Queryer, userID uuid.UUID) (Customer, error)
	FindCustomerByGroupID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (Customer, error)
	FindPaymentMethodByID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (PaymentMethod, error)
	FindCustomerByPaymentMethodID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (Customer, error)
	FindDefaultPaymentMethodForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (PaymentMethod, error)
	FindPlanByID(ctx context.Context, db db.Queryer, planID uuid.UUID) (Plan, error)
	FindDefaultPlan(ctx context.Context, db db.Queryer) (Plan, error)
	FindInvoiceByStripeInvoiceID(ctx context.Context, db db.Queryer, stripeInvoiceID string) (Invoice, error)
	FindCustomerByStripeCustomerID(ctx context.Context, db db.Queryer, stripeCustomerID string) (Customer, error)
	GetSubscribersCountForPlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (int64, error)
	FindAllPlans(ctx context.Context, db db.Queryer) ([]Plan, error)
	FindInvoicesForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) ([]Invoice, error)
	FindInvoicesForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) ([]Invoice, error)
	FindPaymentMethodsForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) ([]PaymentMethod, error)
	FindPaymentMethodsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) ([]PaymentMethod, error)
	GetPaymentMethodsCountForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (int64, error)
}
