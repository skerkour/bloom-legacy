package objects

import (
	"encoding/json"
	"time"

	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/uuid"
)

// Object is an object stored in the local database
type Object struct {
	ID        []byte          `db:"id" json:"id"`
	CreatedAt time.Time       `db:"created_at" json:"createdAt"`
	UpdatedAt time.Time       `db:"updated_at" json:"updatedAt"`
	Type      string          `db:"type" json:"type"`
	Data      json.RawMessage `db:"data" json:"data"`
	OutOfSync bool            `db:"out_of_sync" json:"-"`

	GroupID *uuid.UUID `db:"group_id" json:"groupID"`
}

func GenerateObjectID(username []byte) (ret []byte, err error) {
	randData, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		return
	}
	ret, err = crypto.DeriveKeyFromKey(username, randData, crypto.KeySize512)
	return
}

func ToObject(id []byte, typ string, createdAt, updatedAt time.Time, groupID *uuid.UUID, outOfSync bool, entity interface{}) (ret *Object, err error) {
	jsonData, err := json.Marshal(entity)
	if err != nil {
		return
	}
	ret = &Object{
		ID:        id,
		CreatedAt: createdAt,
		UpdatedAt: updatedAt,
		Type:      typ,
		OutOfSync: outOfSync,
		GroupID:   groupID,
		Data:      jsonData,
	}
	return
}
