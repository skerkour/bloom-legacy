package objects

import (
	"gitlab.com/bloom42/bloom/server/server/errors"
)

type DomainError int

const (
	ErrorInternal DomainError = iota
	ErrorObjectTooLarge
	ErrorOutOfSync
	ErrorObjectNotFound
	ErrorInvalidKeySize
	ErrorInvalidNonceSize
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorObjectTooLarge:
		code = errors.InvalidArgument
		message = "Object is too large."
	case ErrorOutOfSync:
		code = errors.OutOfSync
		message = "Out of sync. Please pull changes before pushing"
	case ErrorObjectNotFound:
		code = errors.NotFound
		message = "Object not found"
	case ErrorInvalidKeySize:
		code = errors.InvalidArgument
		message = "Key size is not valid"
	case ErrorInvalidNonceSize:
		code = errors.InvalidArgument
		message = "Nonce size is not valid"
	}
	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
