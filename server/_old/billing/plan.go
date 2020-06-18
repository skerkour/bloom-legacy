package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/db"
	"gitlab.com/bloom42/gobox/rz"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindPublicPlanById(ctx context.Context, tx *sqlx.Tx, planId uuid.UUID) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1 AND is_public = true"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPublicPlanById: finding plan by id", rz.Err(err),
			rz.String("plan.id", planId.String()))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func FindPlanById(ctx context.Context, tx *sqlx.Tx, planId uuid.UUID) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPlanById: finding plan by id", rz.Err(err),
			rz.String("plan.id", planId.String()))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}

func GetSubscribersCountForPlanId(ctx context.Context, planId uuid.UUID) (int64, error) {
	var ret int64
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT COUNT(*) FROM billing_customers WHERE plan_id = $1"
	err = db.DB.Get(&ret, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.GetSubscribersCountForPlanId: finding plan by id", rz.Err(err),
			rz.String("plan.id", planId.String()))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func GetSubscribersCountForPlanIdTx(ctx context.Context, tx *sqlx.Tx, planId uuid.UUID) (int64, error) {
	var ret int64
	var err error
	logger := rz.FromCtx(ctx)

	queryFindPlan := "SELECT COUNT(*) FROM billing_customers WHERE plan_id = $1"
	err = tx.Get(&ret, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.GetSubscribersCountForPlanIdTx: finding plan by id", rz.Err(err),
			rz.String("plan.id", planId.String()))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlanForUser(ctx context.Context, userId string) (*Plan, error) {
	ret := &Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := `SELECT billing_plans.* FROM billing_plans
	INNER JOIN billing_customers ON billing_plans.id = billing_customers.plan_id
	WHERE billing_customers.user_id = $1`
	err = db.DB.Get(ret, queryFind, userId)
	if err != nil {
		logger.Error("billing.FindPlanForUser: finding plan", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}

func FindPlanForCustomer(ctx context.Context, customer *Customer) (*Plan, error) {
	ret := &Plan{}
	var err error
	logger := rz.FromCtx(ctx)

	queryFind := "SELECT billing_plans.* FROM billing_plans WHERE id = $1"
	err = db.DB.Get(ret, queryFind, customer.PlanID)
	if err != nil {
		logger.Error("billing.FindPlanForCustomer: finding plan", rz.Err(err))
		return ret, NewError(ErrorPlanNotFound)
	}

	return ret, err
}
