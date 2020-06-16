package groups

import (
	"fmt"

	"errors"
)

// ValidateGroupName validates a group name
func ValidateGroupName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.New("Name cannot be empty")
	}

	if nameLen > GROUP_NAME_MAX_LENGTH {
		return fmt.Errorf("Name cannot be longer than %d characters", GROUP_NAME_MAX_LENGTH)
	}

	if nameLen < GROUP_NAME_MIN_LENGTH {
		return fmt.Errorf("Name cannot be shorter than %d characters", GROUP_NAME_MIN_LENGTH)
	}

	return nil
}

// ValidateGroupDescription validates a group description
func ValidateGroupDescription(description string) error {
	descriptionLen := len(description)

	if descriptionLen > GROUP_DESCRIPTION_MAX_LENGTH {
		return fmt.Errorf("Description cannot be longer than %d characters", GROUP_DESCRIPTION_MAX_LENGTH)
	}

	return nil
}
