package validator

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
