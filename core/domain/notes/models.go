package notes

import (
	"time"
)

type Note struct {
	ID         string     `json:"id"`
	CreatedAt  time.Time  `json:"created_at"`
	UpdatedAt  time.Time  `json:"updated_at"`
	ArchivedAt *time.Time `json:"archived_at"`
	Title      string     `json:"title"`
	Body       string     `json:"body"`
	Color      string     `json:"color"`
	IsPinned   bool       `json:"is_pinned"`
}
