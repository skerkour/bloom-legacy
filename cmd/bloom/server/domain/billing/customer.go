package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/cmd/bloom/server/db"
	"gitlab.com/bloom42/lily/rz"
	"gitlab.com/bloom42/lily/uuid"
)

type Customer struct {
	ID        uuid.UUID `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	Email                 string    `json:"email" db:"email"`
	StripeCustomerID      *string   `json:"stripe_customer_id" db:"stripe_customer_id"`
	StripeSubscriptionID  *string   `json:"stripe_subscription_id" db:"stripe_subscription_id"`
	UsedStorage           int64     `json:"used_storage" db:"used_storage"`
	SubscriptionUpdatedAt time.Time `json:"subscription_updated_at" db:"subscription_updated_at"`

	PlanID  uuid.UUID  `json:"plan_id" db:"plan_id"`
	UserID  *uuid.UUID `json:"user_id" db:"user_id"`
	GroupID *uuid.UUID `json:"group_id" db:"group_id"`
}

func FindCustomerByUserId(ctx context.Context, tx *sqlx.Tx, userId uuid.UUID) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = tx.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserId: finding customer", rz.Err(err),
			rz.String("user.id", userId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByUserIdNoTx(ctx context.Context, userId uuid.UUID) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = db.DB.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserIdNoTx: finding customer", rz.Err(err),
			rz.String("user.id", userId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByGroupId(ctx context.Context, tx *sqlx.Tx, groupId uuid.UUID) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = tx.Get(ret, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupId: finding customer", rz.Err(err),
			rz.String("group.id", groupId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByGroupIdNoTx(ctx context.Context, groupId uuid.UUID) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = db.DB.Get(ret, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupIdNoTx: finding customer", rz.Err(err),
			rz.String("group.id", groupId.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByPaymentMethod(ctx context.Context, tx *sqlx.Tx, paymentMethod *PaymentMethod) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE id = $1"
	err = tx.Get(ret, queryFind, paymentMethod.CustomerID)
	if err != nil {
		logger.Error("billing.FindCustomerByPaymentMethod: finding customer", rz.Err(err),
			rz.String("customer.id", paymentMethod.CustomerID.String()))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByStripeCustomerId(ctx context.Context, tx *sqlx.Tx, stripeCustomerId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE stripe_customer_id = $1"
	err = tx.Get(ret, queryFind, stripeCustomerId)
	if err != nil {
		logger.Error("billing.FindCustomerByStripeCustomerId: finding customer", rz.Err(err),
			rz.String("srtripe_customer_id", stripeCustomerId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByStripeCustomerIdNoTx(ctx context.Context, stripeCustomerId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE stripe_customer_id = $1"
	err = db.DB.Get(ret, queryFind, stripeCustomerId)
	if err != nil {
		logger.Error("billing.FindCustomerByStripeCustomerIdNoTx: finding customer", rz.Err(err),
			rz.String("srtripe_customer_id", stripeCustomerId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}
