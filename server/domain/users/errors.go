package users

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

type ErrorCode int

const (
	ErrorSingingIn ErrorCode = iota
)

func NewError(domainCode ErrorCode) errors.Error {
	code := errors.Internal
	message := "Internal error. Please try again."

	switch domainCode {
	case ErrorSingingIn:
		message = "Error signing in. Please try again."
	}
	return errors.New(code, message)
}
