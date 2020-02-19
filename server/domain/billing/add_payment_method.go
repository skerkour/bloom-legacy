package billing

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/stripe/stripe-go"
	stripecustomer "github.com/stripe/stripe-go/customer"
	"github.com/stripe/stripe-go/paymentmethod"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func AddPaymentMethod(ctx context.Context, user *users.User, stripeId string, groupId *string) (*PaymentMethod, error) {
	var ret *PaymentMethod
	var err error
	logger := rz.FromCtx(ctx)
	var customer *Customer
	var stripeCustomerId string
	now := time.Now().UTC()
	isDefault := false

	// clean and validate params
	if user == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return ret, NewError(ErrorAddingPaymentMethod)
	}
	stripeId = strings.TrimSpace(stripeId)
	if !strings.HasPrefix(stripeId, "pm_") {
		return ret, NewError(ErrorStripeIdNotValid)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.AddPaymentMethod: Starting transaction", rz.Err(err))
		return ret, NewError(ErrorAddingPaymentMethod)
	}

	// fetch customer id
	if groupId != nil {
		// check that user is admin of group
		if err = groups.CheckUserIsGroupAdmin(ctx, tx, user.ID, *groupId); err != nil {
			tx.Rollback()
			return ret, err
		}
		customer, err = FindCustomerByGroupId(ctx, tx, *groupId)
		if err != nil {
			tx.Rollback()
			return ret, NewError(ErrorAddingPaymentMethod)
		}
	} else {
		customer, err = FindCustomerByUserId(ctx, tx, user.ID)
		if err != nil {
			tx.Rollback()
			return ret, NewError(ErrorAddingPaymentMethod)
		}
	}

	if customer.StripeCustomerID == nil {
		isDefault = true
		// create stripe customer + update customer
		customerParams := &stripe.CustomerParams{
			PaymentMethod: stripe.String(stripeId),
			Email:         stripe.String(customer.Email),
			InvoiceSettings: &stripe.CustomerInvoiceSettingsParams{
				DefaultPaymentMethod: stripe.String(stripeId),
			},
		}
		stripeCustomer, err := stripecustomer.New(customerParams)
		if err != nil {
			tx.Rollback()
			logger.Error("billing.AddPaymentMethod: creating stripe customer", rz.Err(err))
			return ret, NewError(ErrorAddingPaymentMethod)
		}
		stripeCustomerId = stripeCustomer.ID
		customer.UpdatedAt = now
		customer.StripeCustomerID = &stripeCustomerId
		queryUpdate := `UPDATE billing_customers SET updated_at = $1, stripe_customer_id = $2
			WHERE id = $3`
		_, err = tx.Exec(queryUpdate, customer.UpdatedAt, customer.StripeCustomerID, customer.ID)
		if err != nil {
			tx.Rollback()
			logger.Error("billing.AddPaymentMethod: updating customer with stripe_id", rz.Err(err))
			return ret, NewError(ErrorAddingPaymentMethod)
		}
	} else {
		stripeCustomerId = *customer.StripeCustomerID
	}

	// fetch the stripe payment method
	stripePaymentMethod, err := paymentmethod.Get(stripeId, nil)
	if err != nil || stripePaymentMethod.Card == nil {
		tx.Rollback()
		logger.Error("billing.AddPaymentMethod: fetching stripe payment method", rz.Err(err))
		return ret, NewError(ErrorAddingPaymentMethod)
	}

	// create payment method
	newUuid := uuid.New()
	ret = &PaymentMethod{
		ID:                  newUuid.String(),
		CreatedAt:           now,
		UpdatedAt:           now,
		IsDefault:           isDefault,
		StripeID:            stripePaymentMethod.ID,
		CardLast4:           stripePaymentMethod.Card.Last4,
		CardExpirationMonth: int64(stripePaymentMethod.Card.ExpMonth),
		CardExpirationYear:  int64(stripePaymentMethod.Card.ExpYear),
		CustomerID:          customer.ID,
	}
	queryCreate := `INSERT INTO billing_payment_methods
		(id, created_at, updated_at, is_default, stripe_id, card_last_4, card_expiration_month, card_expiration_year, customer_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`
	_, err = tx.Exec(queryCreate, ret.ID, ret.CreatedAt, ret.UpdatedAt, ret.IsDefault, ret.StripeID, ret.CardLast4,
		ret.CardExpirationMonth, ret.CardExpirationYear, ret.CustomerID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.AddPaymentMethod: inserting new payment method", rz.Err(err))
		return ret, NewError(ErrorAddingPaymentMethod)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.AddPaymentMethod: Committing transaction", rz.Err(err))
		return ret, NewError(ErrorAddingPaymentMethod)
	}

	return ret, err
}
