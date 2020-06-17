package billing

import (
	"context"

	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/uuid"
)

// Repository is the repository for all the entities of the billing subdomain
type Repository interface {
	FindCustomerByUserID(ctx context.Context, db db.Queryer, userID uuid.UUID) (Customer, error)
	FindCustomerByGroupID(ctx context.Context, db db.Queryer, groupID uuid.UUID) (Customer, error)
	UpdateCustomer(ctx context.Context, db db.Queryer, customer Customer) error
	CreatePaymentMethod(ctx context.Context, db db.Queryer, paymentMethod PaymentMethod) error
}
