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
	ErrorRegistrationCodeIsNotValid
	ErrorMaximumVerificationTrialsReached
	ErrorRegistrationCodeExpired
	ErrorUserNotFound
	ErrorInvalidUsernamePasswordCombination
	ErrorSessionNotFound
	ErrorUpdatingProfile
	ErrorUserIsNull
	ErrorAllFieldsAreEmpty
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
	case ErrorRegistrationCodeIsNotValid:
		code = errors.PermissionDenied
		message = "Verification code is not valid."
	case ErrorMaximumVerificationTrialsReached:
		code = errors.PermissionDenied
		message = "Maximum trials reached. Please create a new account."
	case ErrorRegistrationCodeExpired:
		code = errors.PermissionDenied
		message = "Verification code expired. Please create a new account."
	case ErrorUserNotFound:
		code = errors.NotFound
		message = "User not found."
	case ErrorInvalidUsernamePasswordCombination:
		code = errors.PermissionDenied
		message = "Invalid Username / Password combination"
	case ErrorSessionNotFound:
		code = errors.NotFound
		message = "Session not found."
	case ErrorUpdatingProfile:
		message = "Error updating profile. Please try again."
	case ErrorUserIsNull:
		message = "User is null."
	case ErrorAllFieldsAreEmpty:
		code = errors.InvalidArgument
		message = "All fields are empty. Please fix and try again."
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
