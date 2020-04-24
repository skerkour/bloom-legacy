package objects

import (
	"gitlab.com/bloom42/lily/uuid"
)

type Object struct {
	ID             uuid.UUID `db:"id"`
	UpdatedAtState int64     `db:"updated_at_state"`
	Cipher         string    `db:"cipher"`
	Nonce          []byte    `db:"nonce"`
	EncryptedKey   []byte    `db:"encrypted_key"`
	EncryptedData  []byte    `db:"encrypted_data"`

	UserID  *uuid.UUID `db:"user_id"`
	GroupID *uuid.UUID `db:"group_id"`
}
