package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the billing subdomain
type Repository interface {
	UpdateCustomer(ctx context.Context, db db.Queryer, customer Customer) (err error)
	CreatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod PaymentMethod) (err error)
	UpdatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod PaymentMethod) (err error)
	CreateCustomer(ctx context.Context, db db.Queryer, customer Customer) (err error)
	CreateInvoice(ctx context.Context, db db.Queryer, invoice Invoice) (err error)
	UpdateInvoice(ctx context.Context, db db.Queryer, invoice Invoice) (err error)
	CreatePlan(ctx context.Context, db db.Queryer, plan Plan) (err error)
	DeletePlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (err error)
	DeletePaymentMethod(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (err error)
	UpdatePlan(ctx context.Context, db db.Queryer, plan Plan) (err error)

	FindCustomerByUserID(ctx context.Context, db db.Queryer, userID uuid.UUID) (Customer, error)
	FindCustomerByGroupID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (Customer, error)
	FindPaymentMethodByID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (PaymentMethod, error)
	FindCustomerByID(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret Customer, err error)
	FindDefaultPaymentMethodForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret PaymentMethod, err error)
	FindPlanByID(ctx context.Context, db db.Queryer, planID uuid.UUID) (ret Plan, err error)
	FindDefaultPlan(ctx context.Context, db db.Queryer) (ret Plan, err error)
	FindInvoiceByStripeInvoiceID(ctx context.Context, db db.Queryer, stripeInvoiceID string) (ret Invoice, err error)
	FindCustomerByStripeCustomerID(ctx context.Context, db db.Queryer, stripeCustomerID string) (ret Customer, err error)
	GetSubscribersCountForPlan(ctx context.Context, db db.Queryer, planID uuid.UUID) (ret int64, err error)
	FindAllPlans(ctx context.Context, db db.Queryer) (ret []Plan, err error)
	FindInvoicesForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []Invoice, err error)
	FindInvoicesForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []Invoice, err error)
	FindPaymentMethodsForGroup(ctx context.Context, db db.Queryer, groupID uuid.UUID) (ret []PaymentMethod, err error)
	FindPaymentMethodsForUser(ctx context.Context, db db.Queryer, userID uuid.UUID) (ret []PaymentMethod, err error)
	GetPaymentMethodsCountForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret int64, err error)
	FindPlanForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (ret Plan, err error)
}
