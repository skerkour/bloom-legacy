package validator

import (
	"errors"
	"fmt"
	"strings"

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
	if tier != consts.BILLING_FREE_TIER && tier != consts.BILLING_BASIC_TIER &&
		tier != consts.BILLING_PRO_TIER && tier != consts.BILLING_ULTRA_TIER {
		return fmt.Errorf("tier should be either %s, %s, %s or %s", consts.BILLING_FREE_TIER, consts.BILLING_BASIC_TIER,
			consts.BILLING_PRO_TIER, consts.BILLING_ULTRA_TIER)
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

func BillingPlanStripeId(stripeId string) error {
	if !strings.HasPrefix(stripeId, "plan_") {
		return errors.New("stripe_id does not match the pattern \"plan_*\"")
	}

	return nil
}
