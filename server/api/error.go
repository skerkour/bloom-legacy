package api

import (
	"github.com/vektah/gqlparser/v2/gqlerror"
	"gitlab.com/bloom42/bloom/server/errors"
)

// Error is the type representing errors returned any of the HTTP apis
type Error = *gqlerror.Error

// NewError return a new `Error`, with the `Message` field filed
// with `err.Message` and the `Extensions.code` field filed with `err.Code`
func NewError(err error) Error {
	var graphqlError *gqlerror.Error

	switch errValue := err.(type) {
	case *errors.InternalError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "INTERNAL",
			},
		}
	case *errors.AlreadyExistsError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "ALREADY_EXISTS",
			},
		}
	case *errors.AuthenticationRequiredError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "AUTHENTICATION_REQUIRED",
			},
		}
	case *errors.NotFoundError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "NOT_FOUND",
			},
		}
	case *errors.InvalidArgumentError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "INVALID_ARGUMENT",
			},
		}
	case *errors.PermissionDeniedError:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
			Extensions: map[string]interface{}{
				"code": "PERMISSION_DENIED",
			},
		}
	default:
		err := errors.Internal("", err)
		graphqlError = &gqlerror.Error{
			Message: err.Error(),
			Extensions: map[string]interface{}{
				"code": "INTERNAL",
			},
		}

	}
	return graphqlError
}
