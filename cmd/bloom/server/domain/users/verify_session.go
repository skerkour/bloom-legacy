package users

import (
	"errors"

	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/uuid"
)

// VerifySession verifies that secret matches with sessionID and return the session in case of success
func VerifySession(sessionID uuid.UUID, secret []byte) (currentSession *Session, err error) {
	// find session with ID and associated user
	currentSession = GlobalSessionsCache.Get(sessionID)
	if currentSession == nil {
		err = errors.New("Session is not valid")
		return
	}

	err = verifySessionHash(currentSession, secret)
	if err != nil {
		currentSession = nil
		err = errors.New("Session is not valid")
		return
	}

	return
}

func verifySessionHash(session *Session, secret []byte) error {
	hash, err := hashSession(session.ID, session.Salt, secret)
	if err != nil {
		return err
	}

	if !crypto.ConstantTimeCompare(hash, session.Hash) {
		return errors.New("session hash is not valid")
	}

	return nil
}

func hashSession(id uuid.UUID, salt, secret []byte) (hash []byte, err error) {
	message := append(id.Bytes(), salt...)
	hash, err = crypto.DeriveKeyFromKey(secret, message, crypto.KeySize512)
	return
}
