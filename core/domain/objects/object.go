package objects

import (
	"time"

	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/uuid"
)

// Object is an object stored in the local database
type Object struct {
	ID        []byte    `db:"id" json:"id"`
	CreatedAt time.Time `db:"created_at" json:"createdAt"`
	UpdatedAt time.Time ` db:"updated_at" json:"updatedAt"`
	Type      string    `db:"type" json:"type"`
	Data      []byte    `db:"data" json:"data"`
	OutOfSync bool      `db:"out_of_sync" json:"-"`

	GroupID *uuid.UUID `db:"group_id" json:"group_id"`
}

func GenerateObjectID(username []byte) (ret []byte, err error) {
	randData, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		return
	}
	ret, err = crypto.DeriveKeyFromKey(username, randData, crypto.KeySize512)
	return
}
