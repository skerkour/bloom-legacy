package notes

import (
	"context"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/bloom/core/messages"
)

func FindNotes(params messages.NotesFindParams) (Notes, error) {
	ret := Notes{Notes: []objects.Object{}}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_NOTE, params.GroupID)
	if err != nil {
		return ret, err
	}

	for _, obj := range objects {
		var note Note

		err = json.Unmarshal(obj.Data, &note)
		if err != nil {
			return ret, err
		}
		if note.ArchivedAt == nil {
			ret.Notes = append(ret.Notes, obj)
		}
	}

	return ret, nil
}

func FindArchived(params messages.NotesFindParams) (Notes, error) {
	ret := Notes{Notes: []objects.Object{}}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_NOTE, params.GroupID)
	if err != nil {
		return ret, err
	}

	for _, obj := range objects {
		var note Note

		err = json.Unmarshal(obj.Data, &note)
		if err != nil {
			return ret, err
		}
		if note.ArchivedAt != nil {
			ret.Notes = append(ret.Notes, obj)
		}
	}

	return ret, nil
}
