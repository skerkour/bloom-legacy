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

	FindCustomerByUserID(ctx context.Context, db db.Queryer, userID uuid.UUID) (Customer, error)
	FindCustomerByGroupID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (Customer, error)
	FindPaymentMethodByID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (PaymentMethod, error)
	FindCustomerByPaymentMethodID(ctx context.Context, db db.Queryer, paymentMethodID uuid.UUID) (Customer, error)
	FindDefaultPaymentMethodForCustomer(ctx context.Context, db db.Queryer, customerID uuid.UUID) (PaymentMethod, error)
	FindPlanByID(ctx context.Context, db db.Queryer, planID uuid.UUID) (Plan, error)
}
