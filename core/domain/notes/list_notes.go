package notes

import (
	"context"
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/kernel"
	"gitlab.com/bloom42/bloom/core/domain/objects"
)

func ListNotes() (Notes, error) {
	ret := Notes{Notes: []objects.Object{}}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_NOTE)
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

func ListArchived() (Notes, error) {
	ret := Notes{Notes: []objects.Object{}}

	objects, err := objects.FindObjectsByType(context.Background(), nil, kernel.OBJECT_TYPE_NOTE)
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
