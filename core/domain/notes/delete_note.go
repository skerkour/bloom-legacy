package notes

import (
	"context"

	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func DeleteNote(params messages.NotesDeleteParams) (err error) {

	err = objects.DeleteObject(context.Background(), nil, params.ID)

	// request sync
	// objects.SyncChan <- true

	return err
}
