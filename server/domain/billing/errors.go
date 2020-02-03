package billing

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

type DomainError int

const (
	ErrorCreatingCustomer DomainError = iota
)

func NewError(domainError DomainError) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainError {
	case ErrorCreatingCustomer:
	}

	return errors.New(code, message)
}

func NewErrorMessage(domainError DomainError, message string) errors.Error {
	err := NewError(domainError)
	err.Message = message
	return err
}
