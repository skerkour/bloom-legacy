package billing

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

type DomainError int

const (
	ErrorCreatingCustomer DomainError = iota
	ErrorInvalidArgument
	ErrorCreatingPlan
	ErrorDeletingPlan
	ErrorUpdatingPlan
	ErrorAdminRolRequired
	ErrorPlanNotFound
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorCreatingCustomer:
	case ErrorInvalidArgument:
		code = errors.InvalidArgument
		message = "Invalid argument. Please fix before retrying."
	case ErrorCreatingPlan:
		message = "Error creating plan. Please try again."
	case ErrorDeletingPlan:
		message = "Error deleting plan. Please try again."
	case ErrorAdminRolRequired:
		code = errors.PermissionDenied
		message = "Administrator role is required to perform this action."
	case ErrorUpdatingPlan:
		message = "Error updating plan. Please try again."
	case ErrorPlanNotFound:
		code = errors.NotFound
		message = "Plan not found."
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
