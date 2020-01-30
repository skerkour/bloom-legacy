package errors

import (
	"fmt"
)

type Code int

const (
	// Internal should be used for internal errors, where no other code fits
	Internal Code = iota
	// Not
	NotFound
	Unauthenticated
	PermissionDenied
	InvalidArgument
	AlreadyExists
)

type Error struct {
	Code    string
	Message string
}

func New(code Code, message string) Error {
	codeStr := "INTERNAL"

	switch code {
	case NotFound:
		codeStr = "NOT_FOUND"
	case PermissionDenied:
		codeStr = "PERMISSION_DENIED"
	case Unauthenticated:
		codeStr = "UNAUTHENTICATED"
	case InvalidArgument:
		codeStr = "INVALID_ARGUMENT"
	case AlreadyExists:
		codeStr = "ALREADY_EXISTS"
	default:
		codeStr = "INTERNAL"
		if message == "" {
			message = "Internal error. Please try again."
		}
	}
	return Error{
		Code:    codeStr,
		Message: message,
	}
}

func (err Error) Error() string {
	return fmt.Sprintf("%s: %s", err.Code, err.Message)
}
