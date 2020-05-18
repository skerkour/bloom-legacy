package notes

import (
	"context"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func CreateNote(params messages.CreateNoteParams) (*objects.Object, error) {
	var err error
	var ret *objects.Object

	now := time.Now().UTC()
	note := Note{
		ArchivedAt: nil,
		Title:      params.Title,
		Body:       params.Body,
		Color:      params.Color,
		IsFavorite: false,
	}

	id, err := objects.GenerateObjectID([]byte(kernel.Me.Username))
	if err != nil {
		return ret, err
	}

	ret, err = objects.ToObject(id, kernel.OBJECT_TYPE_NOTE, now, now, nil, true, &note)
	if err != nil {
		return ret, err
	}

	err = objects.SaveObject(context.Background(), nil, ret)

	// request sync
	objects.SyncChan <- true

	return ret, err
}
