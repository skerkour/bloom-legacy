package objects

type StoredObject struct {
	ID        string `db:"id" json:"id"`
	Type      string `db:"type" json:"type"`
	Data      []byte `db:"data" json:"data"`
	OutOfSync bool   `db:"out_of_sync" json:"-"`

	GroupID *string `db:"group_id" json:"group_id"`
}
