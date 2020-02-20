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
	ErrorAdminRoleRequired
	ErrorPlanNotFound
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
	ErrorUserIdAndGroupIdCantBeBothNonNull
	ErrorChangingPlan
	ErrorOldPlanIsTheSameAsNewPlan
	ErrorTooMuchStorageUsedForNewPlan
	ErrorPlanStorageCantBeNegative
	ErrorCantDeleteDefaultPlan
	ErrorCantDeletePlanWithSubscribers
	ErrorStripeCustomerIDIsNUll
	ErrorCreatingStripeSubscription
	ErrorInvoiceNotFound
	ErrorCreatingInvoice
	ErrorUpdatingInvoice
	ErrorInvoiceIsNull
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
	case ErrorAdminRoleRequired:
		code = errors.PermissionDenied
		message = "Administrator role is required to perform this action."
	case ErrorUpdatingPlan:
		message = "Error updating plan. Please try again."
	case ErrorPlanNotFound:
		code = errors.NotFound
		message = "Plan not found."
	case ErrorChangingPlan:
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
	case ErrorUserIdAndGroupIdCantBeBothNonNull:
		code = errors.InvalidArgument
		message = "\"user_id\" and \"group_id\" can't be both non null. Please fix and try again."
	case ErrorOldPlanIsTheSameAsNewPlan:
		code = errors.InvalidArgument
		message = "The new plan is the same as the new plan. Please fix and try again."
	case ErrorTooMuchStorageUsedForNewPlan:
		code = errors.InvalidArgument
		message = "Your used storage is superior to the allowed storage for the new plan. Please fix and try again."
	case ErrorPlanStorageCantBeNegative:
		code = errors.InvalidArgument
		message = "Plan storage can't be negative. Please fix and try again."
	case ErrorCantDeleteDefaultPlan:
		code = errors.InvalidArgument
		message = "Can't delete default plan. Please fix and try again."
	case ErrorCantDeletePlanWithSubscribers:
		code = errors.InvalidArgument
		message = "Can't delete plan with subscribers. Please fix and try again."
	case ErrorStripeCustomerIDIsNUll:
		code = errors.InvalidArgument
		message = "Please add a payment method before chaging plan."
	case ErrorInvoiceNotFound:
		code = errors.NotFound
		message = "Invoice not found."
	case ErrorCreatingInvoice:
		message = "Error creating invoice. Please try again."
	case ErrorUpdatingInvoice:
		message = "Error updating invoice. Please try again."
	case ErrorInvoiceIsNull:
		message = "Invoice is null"
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
