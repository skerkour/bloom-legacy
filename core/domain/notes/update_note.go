package notes

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func UpdateNote(note objects.Object) (objects.Object, error) {
	now := time.Now().UTC()
	var err error

	note.UpdatedAt = now
	note.OutOfSync = true

	err = objects.SaveObject(context.Background(), nil, &note)

	// request sync
	// objects.SyncChan <- true

	return note, err
}
