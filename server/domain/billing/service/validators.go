package service

import (
	"fmt"
	"strings"

	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/bloom/server/domain/billing"
	"gitlab.com/bloom42/bloom/server/errors"
)

func validatePlanName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.InvalidArgument("Plan name cannot be empty")
	}

	if nameLen > billing.PlanNameMaxLength {
		return errors.InvalidArgument(fmt.Sprintf("Plan name cannot be longer than %d characters", billing.PlanNameMaxLength))
	}

	if nameLen < billing.PlanNameMinLength {
		return errors.InvalidArgument(fmt.Sprintf("Plan name cannot be shorter than %d characters", billing.PlanNameMinLength))
	}

	return nil
}

func validateProduct(product string) error {
	if product != consts.BILLING_PRODUCT_FREE && product != consts.BILLING_PRODUCT_LITE &&
		product != consts.BILLING_PRODUCT_PRO && product != consts.BILLING_PRODUCT_ULTRA {
		return errors.InvalidArgument(fmt.Sprintf("product should be either %s, %s, %s or %s",
			consts.BILLING_PRODUCT_FREE, consts.BILLING_PRODUCT_LITE,
			consts.BILLING_PRODUCT_PRO, consts.BILLING_PRODUCT_ULTRA))
	}

	return nil
}

func validatePlanPrice(price int64) error {
	if price < 0.0 {
		return errors.InvalidArgument("Plan price can't be less than 0")
	}

	if price > 1000000.0 {
		return errors.InvalidArgument("price can't be more than 1M")
	}

	return nil
}

func validatePlanStripeID(stripeID string) error {
	if !strings.HasPrefix(stripeID, "plan_") {
		return errors.InvalidArgument("stripe_id does not match the pattern \"plan_*\"")
	}

	return nil
}
