package notes

import (
	"time"
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
