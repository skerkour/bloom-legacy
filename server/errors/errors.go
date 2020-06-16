// Package errors contains all the domain error types of the server
package errors

import (
	"fmt"
)

// NotFoundError is a wrapper for an error when something is not found
type NotFoundError struct {
	message string
}

func (err *NotFoundError) Error() string {
	return err.message
}

// NotFound returns a new NotFoundError
func NotFound(message string) *NotFoundError {
	return &NotFoundError{message: message}
}

// InternalError is a wrapper for an error when something bad happened, but we don't want to inform the user
// of the details
type InternalError struct {
	err error
}

func (err *InternalError) Error() string {
	return "Internal error. Please try again."
}

// Internal returns a new InternalError
func Internal(message string, err error) *InternalError {
	return &InternalError{
		err: fmt.Errorf("%s: %w", message, err),
	}
}

// AuthenticationRequiredError is a wrapper for an error when something an unauthenticated person try
// to perform an action which requires authentication
type AuthenticationRequiredError struct {
	message string
}

func (err *AuthenticationRequiredError) Error() string {
	return "Authentication required."
}

// AuthenticationRequired returns a new AuthenticationRequiredError
func AuthenticationRequired(message string) *AuthenticationRequiredError {
	return &AuthenticationRequiredError{message: message}
}

// PermissionDeniedError is a wrapper for an error when an authenticated user try to perform an
// action he does not have the rights for
type PermissionDeniedError struct {
	message string
}

func (err *PermissionDeniedError) Error() string {
	return err.message
}

// PermissionDenied returns a new PermissionDeniedError
func PermissionDenied(message string) *PermissionDeniedError {
	return &PermissionDeniedError{message: message}
}

// InvalidArgumentError is a wrapper for an error when an user provides an invalid input
type InvalidArgumentError struct {
	message string
}

func (err *InvalidArgumentError) Error() string {
	return err.message
}

// InvalidArgument returns a new InvalidArgumentError
func InvalidArgument(message string) *InvalidArgumentError {
	return &InvalidArgumentError{
		message: message,
	}
}

// AlreadyExistsError is a wrapper for an error when something already exists
type AlreadyExistsError struct {
	message string
}

func (err *AlreadyExistsError) Error() string {
	return err.message
}

// AlreadyExists returns a new AlreadyExistsError
func AlreadyExists(message string) *AlreadyExistsError {
	return &AlreadyExistsError{
		message: message,
	}
}

// OutOfSyncError is a wrapper for an error when client is out of sync
type OutOfSyncError struct {
}

func (err *OutOfSyncError) Error() string {
	return "Client is out of sync. Please pull before pushing."
}

// OutOfSync returns a new OutOfSyncError
func OutOfSync() *OutOfSyncError {
	return &OutOfSyncError{}
}
