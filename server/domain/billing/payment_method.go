package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/libs/rz-go"
)

type PaymentMethod struct {
	ID        string    `json:"id" db:"id"`
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	UpdatedAt time.Time `json:"updated_at" db:"updated_at"`

	IsDefault           bool   `json:"is_default" db:"is_default"`
	StripeID            string `json:"stripe_id" db:"stripe_id"`
	CardLast4           string `json:"card_last_4" db:"card_last_4"`
	CardExpirationMonth int64  `json:"card_expiration_month" db:"card_expiration_month"`
	CardExpirationYear  int64  `json:"card_expiration_year" db:"card_expiration_year"`

	CustomerID string `json:"customer_id" db:"customer_id"`
}

func FindPaymentMethodById(ctx context.Context, tx *sqlx.Tx, id string) (*PaymentMethod, error) {
	var ret *PaymentMethod
	var paymentMethod PaymentMethod
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_payment_methods WHERE id = $1"
	err = tx.Get(&paymentMethod, queryFind, id)
	if err != nil {
		logger.Error("billing.FindPaymentMethodById: finding payment method", rz.Err(err),
			rz.String("id", id))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	ret = &paymentMethod
	return ret, nil
}

func FindPaymentMethodByCustomer(ctx context.Context, tx *sqlx.Tx, customer *Customer, isDefault bool) (*PaymentMethod, error) {
	var ret *PaymentMethod
	var paymentMethod PaymentMethod
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT * FROM billing_payment_methods WHERE customer_id = $1 AND is_default = $2"
	err = tx.Get(&paymentMethod, queryFind, customer.ID, isDefault)
	if err != nil {
		logger.Error("billing.FindPaymentMethodByCustomer: finding payment method", rz.Err(err),
			rz.String("customer_id", customer.ID))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	ret = &paymentMethod
	return ret, nil
}

func FindPaymentMethodsByUserId(ctx context.Context, userId string) ([]PaymentMethod, error) {
	ret := []PaymentMethod{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := `SELECT billing_payment_methods.* FROM billing_payment_methods
		INNER JOIN billing_customers ON billing_payment_methods.customer_id = billing_customers.id
		WHERE billing_customers.user_id = $1`
	err = db.DB.Select(&ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindPaymentMethodsByUserId finding payment method", rz.Err(err),
			rz.String("users_id", userId))
		return ret, NewError(ErrorPaymentMethodNotFound)
	}

	return ret, nil
}
