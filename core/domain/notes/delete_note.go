package notes

import (
	"context"

	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteNote(params messages.DeleteNoteParams) (ret messages.Empty, err error) {

	err = objects.DeleteObject(context.Background(), nil, params.ID)

	return ret, err
}
