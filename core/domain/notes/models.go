package notes

import (
	"time"
)

type Note struct {
	ID         string     `json:"id"`
	CreatedAt  time.Time  `json:"createdAt"`
	UpdatedAt  time.Time  `json:"updatedAt"`
	ArchivedAt *time.Time `json:"archivedAt"`
	Title      string     `json:"title"`
	Body       string     `json:"body"`
	Color      string     `json:"color"`
	IsPinned   bool       `json:"isPinned"`
}
