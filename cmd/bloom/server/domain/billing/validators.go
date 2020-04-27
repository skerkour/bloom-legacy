package billing

import (
	"fmt"
	"strings"

	"errors"

	"gitlab.com/bloom42/bloom/common/consts"
)

// ValidatePlanName validate a plan name
func ValidatePlanName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.New("name cannot be empty")
	}

	if nameLen > BILLING_PLAN_NAME_MAX_LENGTH {
		return fmt.Errorf("name cannot be longer than %d characters.", BILLING_PLAN_NAME_MAX_LENGTH)
	}

	if nameLen < BILLING_PLAN_NAME_MIN_LENGTH {
		return fmt.Errorf("name cannot be shorter than %d characters.", BILLING_PLAN_NAME_MIN_LENGTH)
	}

	return nil
}

// ValidateProduct validates a product
func ValidateProduct(product string) error {
	if product != consts.BILLING_PRODUCT_FREE && product != consts.BILLING_PRODUCT_LITE &&
		product != consts.BILLING_PRODUCT_PRO && product != consts.BILLING_PRODUCT_ULTRA {
		return fmt.Errorf("product should be either %s, %s, %s or %s", consts.BILLING_PRODUCT_FREE, consts.BILLING_PRODUCT_LITE,
			consts.BILLING_PRODUCT_PRO, consts.BILLING_PRODUCT_ULTRA)
	}

	return nil
}

// ValidatePlanPrice validate a plan price
func ValidatePlanPrice(price int64) error {
	if price < 0.0 {
		return errors.New("price can't be less than 0")
	}

	if price > 1000000.0 {
		return errors.New("price can't be more than 1M")
	}

	return nil
}

// ValidatePlanStripeID checks is a stripID matchs the `plan_` pattern
func ValidatePlanStripeID(stripeID string) error {
	if !strings.HasPrefix(stripeID, "plan_") {
		return errors.New("stripe_id does not match the pattern \"plan_*\"")
	}

	return nil
}
