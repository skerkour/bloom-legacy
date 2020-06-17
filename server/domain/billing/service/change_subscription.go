package service

import (
	"context"
	"time"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/sub"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/log"
)

func (service *BillingService) ChangeSubscription(ctx context.Context, params billing.ChangeSubscriptionParams) (customer billing.Customer, plan billing.Plan, err error) {
	me, err := service.usersService.Me(ctx)
	if err != nil {
		return
	}
	logger := log.FromCtx(ctx)
	now := time.Now().UTC()
	var stripeSubscription *stripe.Subscription

	if params.UserID == nil && params.GroupID == nil {
		err = billing.ErrUserIdAndGroupIdCantBeBothNonNullrr
		return
	}

	tx, err := service.db.Begin(ctx)
	if err != nil {
		errMessage := "billing.ChangeSubscription: starting transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	if params.UserID != nil {
		if *params.UserID != me.ID && !me.IsAdmin {
			tx.Rollback()
			err = users.ErrPermissionDenied
			return
		}
		customer, err = service.billingRepo.FindCustomerByUserID(ctx, tx, *params.UserID)
		if err != nil {
			tx.Rollback()
			return
		}
	} else if params.GroupID != nil {
		if !me.IsAdmin {
			err = service.groupsService.CheckUserIsGroupAdmin(ctx, tx, me.ID, *params.GroupID)
			if err != nil {
				tx.Rollback()
				return
			}
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
		tx.Rollback()
		errMessage := "billing.ChangeSubscription: stripe customer id is null"
		logger.Error(errMessage)
		err = errors.Internal(errMessage, nil)
		return
	}

	plan, err = service.billingRepo.FindPlanByID(ctx, tx, planId)
	if err != nil {
		tx.Rollback()
		return
	}

	if !me.IsAdmin && !plan.IsPublic {
		tx.Rollback()
		err = billing.ErrPlanNotFound
		return
	}

	oldPlan, err := service.billingRepo.FindPlanByID(ctx, tx, customer.PlanID)
	if err != nil {
		tx.Rollback()
		return
	}

	// check the ability to change plan (used storage)
	if newPlan.ID == oldPlan.ID {
		tx.Rollback()
		err = billing.ErrOldPlanIsTheSameAsNewPlan
		return
	}
	newAllowedStorage := allowedStorageForProduct(newPlan.Product)
	if customer.UsedStorage > newAllowedStorage {
		tx.Rollback()
		err = billing.ErrTooMuchStorageUsedForNewPlan
		return
	}

	if customer.StripeSubscriptionID == nil {
		// create stripe subscription
		firstDayOfMonth := time.Date(now.Year(), now.Month(), 1, 0, 0, 0, 0, time.UTC)
		firstDayofNextMonth := firstDayOfMonth.AddDate(0, 1, 0)
		items := []*stripe.SubscriptionItemsParams{
			{
				Plan: stripe.String(newPlan.StripeID),
			},
		}
		params := &stripe.SubscriptionParams{
			Customer:           stripe.String(*customer.StripeCustomerID),
			Items:              items,
			BillingCycleAnchor: stripe.Int64(firstDayofNextMonth.Unix()),
		}
		// params.AddExpand("latest_invoice.payment_intent")
		stripeSubscription, err = sub.New(params)
		if err != nil {
			tx.Rollback()
			errMessage := "billing.ChangeSubscription: creating stripe subscription"
			logger.Error(errMessage, log.Err("error", err))
			err = errors.Internal(errMessage, err)
			return
		}
		customer.StripeSubscriptionID = &stripeSubscription.ID
	} else {
		// update subscription
		stripeSubscription, err := sub.Get(*customer.StripeSubscriptionID, nil)
		if err != nil {
			tx.Rollback()
			errMessage := "billing.ChangeSubscription: fetching stripe subscription"
			logger.Error(errMessage, log.Err("error", err))
			err = errors.Internal(errMessage, err)
			return
		}
		// currently we choose to create proration, in order to ease comptability, the alternative
		// is to invocie at each changes
		// stripe.String(string(stripe.SubscriptionProrationBehaviorAlwaysInvoice))
		params := &stripe.SubscriptionParams{
			CancelAtPeriodEnd: stripe.Bool(false),
			Items: []*stripe.SubscriptionItemsParams{
				{
					ID:   stripe.String(stripeSubscription.Items.Data[0].ID),
					Plan: stripe.String(newPlan.StripeID),
				},
			},
			BillingCycleAnchorUnchanged: stripe.Bool(true),
			ProrationBehavior:           stripe.String(string(stripe.SubscriptionProrationBehaviorCreateProrations)),
		}
		stripeSubscription, err = sub.Update(stripeSubscription.ID, params)
		if err != nil {
			tx.Rollback()
			errMessage := "billing.ChangeSubscription: updating stripe subscription"
			logger.Error(errMessage, log.Err("error", err))
			err = errors.Internal(errMessage, err)
			return
		}
	}

	customer.UpdatedAt = now
	customer.PlanID = newPlan.ID
	customer.SubscriptionUpdatedAt = now
	err = service.billingRepo.UpdateCustomer(ctx, tx, customer)
	if err != nil {
		tx.Rollback()
		return
	}

	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		errMessage := "billing.ChangeSubscription: committing transaction"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}
	return
}
