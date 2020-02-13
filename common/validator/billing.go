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

func BillingProduct(product string) error {
	if product != consts.BILLING_PRODUCT_FREE && product != consts.BILLING_PRODUCT_BASIC &&
		product != consts.BILLING_PRODUCT_PRO && product != consts.BILLING_PRODUCT_ULTRA {
		return fmt.Errorf("product should be either %s, %s, %s or %s", consts.BILLING_PRODUCT_FREE, consts.BILLING_PRODUCT_BASIC,
			consts.BILLING_PRODUCT_PRO, consts.BILLING_PRODUCT_ULTRA)
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
