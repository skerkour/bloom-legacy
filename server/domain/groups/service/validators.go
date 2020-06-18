package service

import (
	"fmt"

	"gitlab.com/bloom42/bloom/server/domain/groups"
	"gitlab.com/bloom42/bloom/server/errors"
)

func validateCreateGroup(name, description string) (err error) {
	if err = validateGroupName(name); err != nil {
		return
	}

	if err = validateGroupDescription(description); err != nil {
		return
	}

	return
}

func validateGroupName(name string) error {
	nameLen := len(name)

	if nameLen == 0 {
		return errors.InvalidArgument("Name cannot be empty")
	}

	if nameLen > groups.GroupNameMaxLength {
		return errors.InvalidArgument(fmt.Sprintf("Name cannot be longer than %d characters", groups.GroupNameMaxLength))
	}

	if nameLen < groups.GroupeNameMinLength {
		return errors.InvalidArgument(fmt.Sprintf("Name cannot be shorter than %d characters", groups.GroupeNameMinLength))
	}

	return nil
}

func validateGroupDescription(description string) error {
	descriptionLen := len(description)

	if descriptionLen > groups.GroupDescriptionMaxLength {
		return errors.InvalidArgument(fmt.Sprintf("Description cannot be longer than %d characters", groups.GroupDescriptionMaxLength))
	}

	return nil
}
