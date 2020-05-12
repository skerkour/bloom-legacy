package notes

import (
	"encoding/json"
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
	"gitlab.com/bloom42/lily/uuid"
)

type Notes struct {
	Notes []Note `json:"notes"`
}

type Note struct {
	ArchivedAt *time.Time `json:"archivedAt" db:"archived_at"`
	Title      string     `json:"title" db:"title"`
	Body       string     `json:"body" db:"body"`
	Color      string     `json:"color" db:"color"`
	IsPinned   bool       `json:"isPinned" db:"is_pinned"`
}

func NoteToObject(id []byte, createdAt, updatedAt time.Time, groupID *uuid.UUID, outOfSync bool, note *Note) (ret *objects.Object, err error) {
	jsonData, err := json.Marshal(note)
	if err != nil {
		return
	}
	ret = &objects.Object{
		ID:        id,
		CreatedAt: createdAt,
		UpdatedAt: updatedAt,
		Type:      "NOTE_TYPE",
		OutOfSync: outOfSync,
		GroupID:   groupID,
		Data:      jsonData,
	}
	return
}
