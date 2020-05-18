package notes

import (
	"time"

	"gitlab.com/bloom42/bloom/core/domain/objects"
)

type Notes struct {
	Notes []objects.Object `json:"notes"`
}

type Note struct {
	ArchivedAt *time.Time `json:"archivedAt"`
	Title      string     `json:"title"`
	Body       string     `json:"body"`
	Color      string     `json:"color"`
	IsFavorite bool       `json:"isFavorite"`
}
