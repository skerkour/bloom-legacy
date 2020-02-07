package billing

import (
	"context"
	"time"

	"github.com/jmoiron/sqlx"
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
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &paymentMethod
	return ret, err
}
