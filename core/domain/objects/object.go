package objects

import (
	"time"
)

// StoredObject is an object stored in the local database
type StoredObject struct {
	ID        []byte    `db:"id" json:"id"`
	CreatedAt time.Time `db:"created_at" json:"createdAt"`
	UpdatedAt time.Time ` db:"updated_at" json:"updatedAt"`
	Type      string    `db:"type" json:"type"`
	Data      []byte    `db:"data" json:"data"`
	OutOfSync bool      `db:"out_of_sync" json:"-"`

	GroupID *string `db:"group_id" json:"group_id"`
}
