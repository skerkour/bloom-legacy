package validator

import (
	"errors"
	"fmt"
	"gitlab.com/bloom42/bloom/common/consts"
)

func GroupName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.New("name cannot be empty")
	}

	if nameLen > consts.GROUP_NAME_MAX_LENGTH {
		return fmt.Errorf("name cannot be longer than %d characters.", consts.GROUP_NAME_MAX_LENGTH)
	}

	if nameLen < consts.GROUP_NAME_MIN_LENGTH {
		return fmt.Errorf("name cannot be shorter than %d characters.", consts.GROUP_NAME_MIN_LENGTH)
	}

	return nil
}

func GroupDescription(description string) error {
	descriptionLen := len(description)

	if descriptionLen > consts.GROUP_DESCRIPTION_MAX_LENGTH {
		return fmt.Errorf("description cannot be longer than %d characters.", consts.GROUP_DESCRIPTION_MAX_LENGTH)
	}

	return nil
}
