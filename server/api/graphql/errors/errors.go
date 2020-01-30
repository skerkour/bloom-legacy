package errors

import (
	"github.com/vektah/gqlparser/gqlerror"
	"gitlab.com/bloom42/bloom/server/errors"
)

type Error = *gqlerror.Error

// New return a new `Error`, with the `Message` field filed
// with `err.Message` and the `Extensions.code` field filed with `err.Code`
func New(err error) Error {
	var graphqlError *gqlerror.Error

	switch errValue := err.(type) {
	case errors.Error:
		graphqlError = &gqlerror.Error{
			Message: errValue.Message,
			Extensions: map[string]interface{}{
				"code": errValue.Code,
			},
		}
	default:
		graphqlError = &gqlerror.Error{
			Message: errValue.Error(),
		}
	}
	return graphqlError
}
