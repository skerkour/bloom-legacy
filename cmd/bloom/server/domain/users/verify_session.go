package users

import (
	"errors"

	"gitlab.com/bloom42/lily/crypto"
	"gitlab.com/bloom42/lily/uuid"
)

func VerifySession(sessionID uuid.UUID, secret []byte) (currentSession *Session, err error) {
	// find session with ID and associated user
	currentSession = GlobalSessionsCache.Get(sessionID)
	if currentSession == nil {
		err = errors.New("Session is not valid")
		return
	}

	err = verifySessionHash(currentSession, secret)
	if err != nil {
		err = errors.New("Session is not valid")
		return
	}

	return
}

func verifySessionHash(session *Session, secret []byte) error {
	message := append([]byte(session.ID.String()), session.Salt...)
	hash, err := crypto.DeriveKeyFromKey(secret, message, crypto.KeySize512)
	if err != nil {
		return err
	}

	if !crypto.ConstantTimeCompare(hash, session.Hash) {
		return errors.New("session hash is not valid")
	}

	return nil
}
