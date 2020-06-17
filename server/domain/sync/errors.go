package sync

import (
	"gitlab.com/bloom42/bloom/server/errors"
)

var (
	ErrObjectTooLarge         = errors.InvalidArgument("Object is too large.")
	ErrOutOfSync              = errors.OutOfSync()
	ErrObjectNotFound         = errors.NotFound("Object not found")
	ErrInvalidObjectKeySize   = errors.InvalidArgument("Object's key size is not valid")
	ErrInvalidObjectNonceSize = errors.InvalidArgument("Object's nonce size is not valid")
	ErrInvalidState           = errors.InvalidArgument("State is not valid")
)
