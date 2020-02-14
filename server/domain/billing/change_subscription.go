package billing

import (
	"context"
	"time"

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
	} else { // groupId != nil
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
	// update customer
	queryUpdate := "UPDATE billing_customers SET updated_at = $1, plan_id = $2, subscription_updated_at = $3 WHERE id = $4"

	// update oldDefaultPaymentMethod

	customer.UpdatedAt = now
	customer.PlanID = newPlan.ID
	customer.SubscriptionUpdatedAt = now
	_, err = tx.Exec(queryUpdate, customer.UpdatedAt, customer.PlanID, customer.SubscriptionUpdatedAt, customer.ID)
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
