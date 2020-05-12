package contacts

import (
	"context"

	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteContact(params messages.DeleteContactParams) (err error) {

	err = objects.DeleteObject(context.Background(), nil, params.ID)

	return err
}
