package service

import (
	"context"
	"strings"
	"time"

	"github.com/stripe/stripe-go"
	stripecustomer "github.com/stripe/stripe-go/customer"
	"github.com/stripe/stripe-go/paymentmethod"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func (service *BillingService) AddPaymentMethod(ctx context.Context, params billing.AddPaymentMethodParams) (ret billing.PaymentMethod, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)
	now := time.Now().UTC()

	// clean and validate params
	params.StripeID = strings.TrimSpace(params.StripeID)
	if !strings.HasPrefix(params.StripeID, "pm_") {
		err = billing.ErrStripeIDIsNotValid
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.AddPaymentMethod: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	// fetch customer id
	if params.GroupID != nil {
		err = service.groupsService.CheckUserIsGroupAdmin(ctx, tx, me.ID, *params.GroupID)
		if err != nil {
			tx.Rollback()
			return
		}
		customer, err = service.billingRepo.FindCustomerByGroupID(ctx, tx, *params.GroupID)
		if err != nil {
			tx.Rollback()
			return
		}
	} else {
		customer, err = service.billingRepo.FindCustomerByUserID(ctx, tx, me.ID)
		if err != nil {
			tx.Rollback()
			return
		}
	}

	if customer.StripeCustomerID == nil {
		isDefault = true
		// create stripe customer + update customer
		customerParams := &stripe.CustomerParams{
			PaymentMethod: stripe.String(params.StripeID),
			Email:         stripe.String(customer.Email),
			InvoiceSettings: &stripe.CustomerInvoiceSettingsParams{
				DefaultPaymentMethod: stripe.String(params.StripeID),
			},
		}
		stripeCustomer, err := stripecustomer.New(customerParams)
		if err != nil {
			tx.Rollback()
			errMessage := "billing.AddPaymentMethod: creating stripe customer"
			logger.Error(errMessage, log.Err("error", err))
			err = errors.Internal(errMessage, err)
			return
		}
		stripeCustomerId = stripeCustomer.ID
		customer.UpdatedAt = now
		customer.StripeCustomerID = &stripeCustomerId
		err = service.billingRepo.UpdateCustomer(ctx, tx, customer)
		if err != nil {
			tx.Rollback()
			return
		}
	} else {
		stripeCustomerId = *customer.StripeCustomerID
	}

	stripePaymentMethod, err := paymentmethod.Get(params.StripeID, nil)
	if err != nil || stripePaymentMethod.Card == nil {
		tx.Rollback()
		errMessage := "billing.AddPaymentMethod: fetching stripe payment method"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	ret = PaymentMethod{
		ID:                  uuid.New(),
		CreatedAt:           now,
		UpdatedAt:           now,
		IsDefault:           isDefault,
		StripeID:            stripePaymentMethod.ID,
		CardLast4:           stripePaymentMethod.Card.Last4,
		CardExpirationMonth: int64(stripePaymentMethod.Card.ExpMonth),
		CardExpirationYear:  int64(stripePaymentMethod.Card.ExpYear),
		CustomerID:          customer.ID,
	}
	err = service.billingRepo.CreatePaymentMethod(ctx, tx, ret)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.AddPaymentMethod: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
