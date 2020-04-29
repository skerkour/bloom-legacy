package sync

import (
	"gitlab.com/bloom42/lily/uuid"
)

// Object is an object
type Object struct {
	ID             []byte `db:"id"`
	UpdatedAtState int64  `db:"updated_at_state"`
	Algorithm      string `db:"algorithm"`
	Nonce          []byte `db:"nonce"`
	EncryptedKey   []byte `db:"encrypted_key"`
	EncryptedData  []byte `db:"encrypted_data"`

	UserID  *uuid.UUID `db:"user_id"`
	GroupID *uuid.UUID `db:"group_id"`
}

// APIObject represents an object as exchange through the API
type APIObject struct {
	ID            []byte `db:"id"`
	Algorithm     string `db:"algorithm"`
	Nonce         []byte `db:"nonce"`
	EncryptedKey  []byte `db:"encrypted_key"`
	EncryptedData []byte `db:"encrypted_data"`
}
