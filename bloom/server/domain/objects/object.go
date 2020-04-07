package objects

type Object struct {
	ID             string `db:"id"`
	UpdatedAtState int64  `db:"updated_at_state"`
	Cipher         string `db:"cipher"`
	Nonce          []byte `db:"nonce"`
	EncryptedKey   []byte `db:"encrypted_key"`
	EncryptedData  []byte `db:"encrypted_data"`

	UserID  *string `db:"user_id"`
	GroupID *string `db:"group_id"`
}
