package validator

import (
	"errors"
	"fmt"

	"gitlab.com/bloom42/bloom/common/consts"
)

func BillingPlanName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.New("name cannot be empty")
	}

	if nameLen > consts.BILLING_PLAN_NAME_MAX_LENGTH {
		return fmt.Errorf("name cannot be longer than %d characters.", consts.BILLING_PLAN_NAME_MAX_LENGTH)
	}

	if nameLen < consts.BILLING_PLAN_NAME_MIN_LENGTH {
		return fmt.Errorf("name cannot be shorter than %d characters.", consts.BILLING_PLAN_NAME_MIN_LENGTH)
	}

	return nil
}

func BillingPlanTier(tier string) error {
	if tier != "FREE" && tier != "BASIC" && tier != "PRO" && tier != "ULTRA" {
		return errors.New("tier should be either FREE, BASIC, PRO or ULTRA")
	}

	return nil
}

func BillingPlanPrice(price float64) error {
	if price < 0.0 {
		return errors.New("price can't be less than 0")
	}

	if price > 1000000.0 {
		return errors.New("price can't be more than 1M")
	}

	return nil
}
