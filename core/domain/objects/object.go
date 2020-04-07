package objects

type Object struct {
	ID          string `db:"id"`
	Type        string `db:"type"`
	Data        string `db:"data"`
	IsOutOfSync bool   `db:"is_out_of_sync"`

	GroupID *string `db:"group_id"`
}

type DecryptedObjectData struct {
	Type string `json:"string"`
	Data string `json:"data"`
}

type EncryptedObject struct {
	ID             string `json:"id"`
	UpdatedAtState string `json:"updatedAtState"`
	Cipher         string `json:"cipher"`
	EncryptedKey   []byte `json:"encryptedKey"`
	Nonce          []byte `json:"nonce"`
	EncryptedData  []byte `json:"encryptedData"`
}
