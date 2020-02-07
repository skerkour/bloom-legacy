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
	ErrorChangingBillingPlan
	ErrorStripeIdNotValid
	ErrorAddingPaymentMethod
	ErrorRemovingPaymentMethod
	ErrorPaymentMethodNotFound
	ErrorCustomerNotFound
	ErrorRemovingDefaultPaymentMethod
	ErrorRemovingDefaultPaymentMethodOnNonFreePlan
	ErrorFindingPlans
	ErrorChangingDefaultPaymentMethod
	ErrorPaymentMethodIsAlreadyDefault
	ErrorUserIsNull
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
	case ErrorChangingBillingPlan:
		message = "Error changing plan. Please try again."
	case ErrorStripeIdNotValid:
		code = errors.InvalidArgument
		message = "\"stripe_id\" argument is not valid."
	case ErrorAddingPaymentMethod:
		message = "Error adding payment method. Please try again."
	case ErrorRemovingPaymentMethod:
		message = "Error removing payment method. Please try again."
	case ErrorPaymentMethodNotFound:
		code = errors.NotFound
		message = "Payment method not found."
	case ErrorCustomerNotFound:
		code = errors.NotFound
		message = "Customer not found."
	case ErrorRemovingDefaultPaymentMethod:
		code = errors.PermissionDenied
		message = "Please change your default payment method before removing it."
	case ErrorRemovingDefaultPaymentMethodOnNonFreePlan:
		code = errors.PermissionDenied
		message = "Please change your plan to FREE before removeing your payment method."
	case ErrorFindingPlans:
		message = "Error finding plans. Please try again."
	case ErrorChangingDefaultPaymentMethod:
		message = "Error changing payment method. Please try again."
	case ErrorPaymentMethodIsAlreadyDefault:
		code = errors.InvalidArgument
		message = "Payment method is already the default. Please change and try again."
	case ErrorUserIsNull:
		message = "User is null"
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
