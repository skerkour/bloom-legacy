package objects

type StoredObject struct {
	ID        string `db:"id"`
	Type      string `db:"type"`
	Data      string `db:"data"`
	OutOfSync bool   `db:"out_of_sync"`

	GroupID *string `db:"group_id"`
}

type DecryptedObjectData struct {
	Type string `json:"string"`
	Data string `json:"data"`
}
