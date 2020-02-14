package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type Customer struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Email         string    `json:"email" db:"email"`
	StripeID      *string   `json:"stripe_id" db:"stripe_id"`
	UsedStorage   int64     `json:"used_storage" db:"used_storage"`
	PlanUpdatedAt time.Time `json:"plan_updated_at" db:"plan_updated_at"`

	PlanID  string  `json:"plan_id" db:"plan_id"`
	UserID  *string `json:"user_id" db:"user_id"`
	GroupID *string `json:"group_id" db:"group_id"`
}

func FindCustomerByUserId(ctx context.Context, tx *sqlx.Tx, userId string) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = tx.Get(&customer, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserId: finding customer", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	ret = &customer
	return ret, err
}

func FindCustomerByUserIdNoTx(ctx context.Context, userId string) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = db.DB.Get(&customer, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserIdNoTx: finding customer", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	ret = &customer
	return ret, err
}

func FindCustomerByGroupId(ctx context.Context, tx *sqlx.Tx, groupId string) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = tx.Get(&customer, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupId: finding customer", rz.Err(err),
			rz.String("id", groupId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	ret = &customer
	return ret, err
}

func FindCustomerByPaymentMethod(ctx context.Context, tx *sqlx.Tx, paymentMethod *PaymentMethod) (*Customer, error) {
	var ret *Customer
	var customer Customer
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE id = $1"
	err = tx.Get(&customer, queryFind, paymentMethod.CustomerID)
	if err != nil {
		logger.Error("billing.FindCustomerByPaymentMethod: finding customer", rz.Err(err),
			rz.String("id", paymentMethod.CustomerID))
		return ret, NewError(ErrorCustomerNotFound)
	}

	ret = &customer
	return ret, err
}
