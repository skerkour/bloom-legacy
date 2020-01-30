package errors

import (
	"github.com/vektah/gqlparser/gqlerror"
)

type Code int

const (
	// Internal should be used for internal errors, where no other code fits
	Internal Code = iota
	// Not
	NotFound
	Unauthenticated
	PermissionDenied
)

// New return a new `github.com/vektah/gqlparser/gqlerror.Error`, with the `Message` field filed
// with `message` and the `Extensions.code` fields filed with a string representation of `code`
func New(code Code, message string) *gqlerror.Error {
	codeStr := "INTERNAL"

	switch code {
	case NotFound:
		codeStr = "NOT_FOUND"
	case PermissionDenied:
		codeStr = "PERMISSION_DENIED"
	case Unauthenticated:
		codeStr = "UNAUTHENTICATED"
	}

	return &gqlerror.Error{
		Message: message,
		Extensions: map[string]interface{}{
			"code": codeStr,
		},
	}
}
