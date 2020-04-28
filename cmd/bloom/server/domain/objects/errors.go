package objects

import (
	"gitlab.com/bloom42/bloom/cmd/bloom/server/errors"
)

type DomainError int

const (
	ErrorInternal DomainError = iota
	ErrorObjectTooLarge
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorObjectTooLarge:
		code = errors.InvalidArgument
		message = "Object is too large."
	}
	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
