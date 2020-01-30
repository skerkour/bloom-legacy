package users

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

type DomainError int

const (
	ErrorSingingIn DomainError = iota
	ErrorInvalidArgument
	ErrorCreatingPendingUser
	ErrorCompletingRegistration
	ErrorUsernameAlreadyExists
	ErrorEmailAlreadyExists
	ErrorDeletingSession
	ErrorVerifyingPendingUser
	ErrorSendingNewRegistrationCode
	ErrorVerificationCodeIsNotValid
	ErrorMaximumVerificationTrialsReached
	ErrorVerificationCodeExpired
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorSingingIn:
		message = "Error signing in. Please try again."
	case ErrorInvalidArgument:
		code = errors.InvalidArgument
		message = "Invalid argument. Please fix before retrying."
	case ErrorCreatingPendingUser:
		message = "Error creating new account. Please try again."
	case ErrorCompletingRegistration:
		message = "Error completing registration. Please try again."
	case ErrorUsernameAlreadyExists:
		code = errors.AlreadyExists
		message = "An account with this username already exsits. Please change and try again."
	case ErrorEmailAlreadyExists:
		code = errors.AlreadyExists
		message = "An account with this email already exsits. Please change and try again."
	case ErrorDeletingSession:
		message = "Error deleting session. Please try again."
	case ErrorVerifyingPendingUser:
		message = "Error verifying account. Please try again."
	case ErrorSendingNewRegistrationCode:
		message = "Error sending new verification code. Please try again."
	case ErrorVerificationCodeIsNotValid:
		code = errors.PermissionDenied
		message = "Verification code is not valid."
	case ErrorMaximumVerificationTrialsReached:
		code = errors.PermissionDenied
		message = "Maximum trials reached. Please create a new account."
	case ErrorVerificationCodeExpired:
		code = errors.PermissionDenied
		message = "Verification code expired. Please create a new account."
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
