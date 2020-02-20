package billing

import (
	"context"
	"time"

	"github.com/stripe/stripe-go"
	"github.com/stripe/stripe-go/sub"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

// ChangeSubscription Updates plan for customer
func ChangeSubscription(ctx context.Context, actor *users.User, userId, groupId *string, planId string) (*Customer, *Plan, error) {
	logger := rz.FromCtx(ctx)
	var customer *Customer
	var err error
	var retPlan *Plan
	now := time.Now().UTC()
	var stripeSubscription *stripe.Subscription

	// validate params
	if actor == nil {
		logger.Error("", rz.Err(NewError(ErrorUserIsNull)))
		return customer, retPlan, NewError(ErrorChangingPlan)
	}

	if userId != nil && groupId != nil {
		return customer, retPlan, NewError(ErrorUserIdAndGroupIdCantBeBothNonNull)
	}

	// start DB transaction
	tx, err := db.DB.Beginx()
	if err != nil {
		logger.Error("billing.ChangeSubscription: Starting transaction", rz.Err(err))
		return customer, retPlan, NewError(ErrorChangingPlan)
	}

	if userId != nil {
		if *userId != actor.ID && !actor.IsAdmin {
			tx.Rollback()
			return customer, retPlan, NewError(ErrorAdminRoleRequired)
		}
		customer, err = FindCustomerByUserId(ctx, tx, *userId)
		if err != nil {
			tx.Rollback()
			return customer, retPlan, err
		}
	} else if groupId != nil { // groupId != nil
		if !actor.IsAdmin {
			if err = groups.CheckUserIsGroupAdmin(ctx, tx, actor.ID, *groupId); err != nil {
				tx.Rollback()
				return customer, retPlan, err
			}
		}
		customer, err = FindCustomerByGroupId(ctx, tx, *groupId)
		if err != nil {
			tx.Rollback()
			return customer, retPlan, err
		}
	} else {
		customer, err = FindCustomerByUserId(ctx, tx, actor.ID)
		if err != nil {
			tx.Rollback()
			return customer, retPlan, err
		}
	}

	if customer.StripeCustomerID == nil {
		if err != nil {
			tx.Rollback()
			return customer, retPlan, NewError(ErrorStripeCustomerIDIsNUll)
		}
	}

	newPlan, err := FindPlanById(ctx, tx, planId)
	if err != nil {
		tx.Rollback()
		logger.Warn("billing.ChangeSubscription:f inding newPlan", rz.Err(err), rz.String("id", planId))
		return customer, retPlan, err
	}

	if !actor.IsAdmin && !newPlan.IsPublic {
		tx.Rollback()
		return customer, retPlan, NewError(ErrorPlanNotFound)
	}

	oldPlan, err := FindPlanById(ctx, tx, customer.PlanID)
	if err != nil {
		tx.Rollback()
		logger.Warn("billing.ChangeSubscription: finding old plan", rz.Err(err), rz.String("id", customer.PlanID))
		return customer, retPlan, err
	}

	// check the ability to change plan (used storage)
	if newPlan.ID == oldPlan.ID {
		tx.Rollback()
		return customer, retPlan, NewError(ErrorOldPlanIsTheSameAsNewPlan)
	}
	newAllowedStorage := GetAllowedStorageFromPlanProduct(newPlan.Product)
	if customer.UsedStorage > newAllowedStorage {
		tx.Rollback()
		return customer, retPlan, NewError(ErrorTooMuchStorageUsedForNewPlan)
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
			return customer, retPlan, NewError(ErrorCreatingStripeSubscription)
		}
		customer.StripeSubscriptionID = &stripeSubscription.ID
	} else {
		// update subscription
		stripeSubscription, err := sub.Get(*customer.StripeSubscriptionID, nil)
		if err != nil {
			tx.Rollback()
			return customer, retPlan, NewError(ErrorCreatingStripeSubscription)
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
			return customer, retPlan, NewError(ErrorCreatingStripeSubscription)
		}
	}

	// update customer
	customer.UpdatedAt = now
	customer.PlanID = newPlan.ID
	customer.SubscriptionUpdatedAt = now
	queryUpdate := `UPDATE billing_customers
		SET updated_at = $1, plan_id = $2, subscription_updated_at = $3, stripe_subscription_id = $4
		WHERE id = $5`
	_, err = tx.Exec(queryUpdate, customer.UpdatedAt, customer.PlanID, customer.SubscriptionUpdatedAt,
		customer.StripeSubscriptionID, customer.ID)
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeSubscription: customer", rz.Err(err))
		return customer, retPlan, NewError(ErrorChangingPlan)
	}

	// commit db transaction
	err = tx.Commit()
	if err != nil {
		tx.Rollback()
		logger.Error("billing.ChangeSubscription: Committing transaction", rz.Err(err))
		return customer, retPlan, NewError(ErrorChangingPlan)
	}

	retPlan = newPlan
	return customer, retPlan, nil
}
