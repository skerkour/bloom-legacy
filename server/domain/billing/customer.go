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

	Email                 string    `json:"email" db:"email"`
	StripeCustomerID      *string   `json:"stripe_customer_id" db:"stripe_customer_id"`
	StripeSubscriptionID  *string   `json:"stripe_subscription_id" db:"stripe_subscription_id"`
	UsedStorage           int64     `json:"used_storage" db:"used_storage"`
	SubscriptionUpdatedAt time.Time `json:"subscription_updated_at" db:"subscription_updated_at"`

	PlanID  string  `json:"plan_id" db:"plan_id"`
	UserID  *string `json:"user_id" db:"user_id"`
	GroupID *string `json:"group_id" db:"group_id"`
}

func FindCustomerByUserId(ctx context.Context, tx *sqlx.Tx, userId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = tx.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserId: finding customer", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByUserIdNoTx(ctx context.Context, userId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE user_id = $1"
	err = db.DB.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindCustomerByUserIdNoTx: finding customer", rz.Err(err),
			rz.String("id", userId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByGroupId(ctx context.Context, tx *sqlx.Tx, groupId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = tx.Get(ret, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupId: finding customer", rz.Err(err),
			rz.String("id", groupId))
		return ret, NewError(ErrorCustomerNotFound)
	}

	return ret, err
}

func FindCustomerByGroupIdNoTx(ctx context.Context, groupId string) (*Customer, error) {
	ret := &Customer{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_customers WHERE group_id = $1"
	err = db.DB.Get(ret, queryFind, groupId)
	if err != nil {
		logger.Error("billing.FindCustomerByGroupIdNoTx: finding customer", rz.Err(err),
			rz.String("group_id", groupId))
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
			rz.String("customer_id", paymentMethod.CustomerID))
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
