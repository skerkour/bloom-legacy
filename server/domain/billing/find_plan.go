package billing

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/libs/rz-go"
)

func FindPlanById(ctx context.Context, tx *sqlx.Tx, user *users.User, planId string) (*Plan, error) {
	var ret *Plan
	var plan Plan
	var err error
	logger := rz.FromCtx(ctx)

	if !user.IsAdmin {
		return ret, NewError(ErrorAdminRolRequired)
	}

	queryFindPlan := "SELECT * FROM billing_plans WHERE id = $1"
	err = tx.Get(&plan, queryFindPlan, planId)
	if err != nil {
		logger.Error("billing.FindPlanById: finding plan by id", rz.Err(err),
			rz.String("id", planId))
		return ret, NewError(ErrorPlanNotFound)
	}

	ret = &plan
	return ret, err
}
