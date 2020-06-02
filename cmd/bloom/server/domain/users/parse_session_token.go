package users

import (
	"encoding/base64"
	"errors"

	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/uuid"
)

// ParseSessionToken parse the given token and returns the session id and secret
func ParseSessionToken(token string) (id uuid.UUID, secret []byte, err error) {
	var tokenBytes []byte

	tokenBytes, err = base64.StdEncoding.DecodeString(token)
	if err != nil {
		return
	}

	if len(tokenBytes) != uuid.Size+crypto.KeySize512 {
		err = errors.New("Session is not valid")
		return
	}

	sessionIDBytes := tokenBytes[:uuid.Size]
	secret = tokenBytes[uuid.Size:]

	id, err = uuid.FromBytes(sessionIDBytes)

	return
}
