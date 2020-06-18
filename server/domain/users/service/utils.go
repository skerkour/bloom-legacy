package service

import (
	"context"
	"encoding/base64"
	"time"
	"unicode"

	"gitlab.com/bloom42/bloom/server/domain/users"
	"gitlab.com/bloom42/bloom/server/errors"
	"gitlab.com/bloom42/gobox/crypto"
	"gitlab.com/bloom42/gobox/log"
	"gitlab.com/bloom42/gobox/uuid"
)

func newSession(ctx context.Context, userID uuid.UUID, device users.SessionDevice) (session users.Session, token string, err error) {
	logger := log.FromCtx(ctx)

	sessionID := uuid.New()
	now := time.Now().UTC()

	sessionSecret, err := crypto.RandBytes(crypto.KeySize512)
	if err != nil {
		errMessage := "users.newSession: generating random session secret"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	sessionHash, err := hashSession(sessionID, sessionSecret)
	if err != nil {
		errMessage := "users.newSession: hashing session secret"
		logger.Error(errMessage, log.Err("error", err))
		err = errors.Internal(errMessage, err)
		return
	}

	token = encodeSessionToken(sessionID, sessionSecret)

	session = users.Session{
		ID:        sessionID,
		CreatedAt: now,
		UpdatedAt: now,

		Hash:       sessionHash,
		DeviceOS:   device.OS,
		DeviceType: device.Type,

		UserID: userID,
	}
	return
}

func hashSession(id uuid.UUID, secret []byte) (hash []byte, err error) {
	hash, err = crypto.DeriveKeyFromKey(secret, id.Bytes(), crypto.KeySize512)
	if err != nil {
		err = errors.Internal("users.hashSession: crypto.DeriveKeyFromKey", err)
	}

	return
}

func verifySessionSecret(ctx context.Context, session users.Session, secret []byte) error {
	hash, err := hashSession(session.ID, secret)
	if err != nil {
		return err
	}

	if !crypto.ConstantTimeCompare(hash, session.Hash) {
		return users.ErrInvalidSession
	}

	return nil
}

func encodeSessionToken(id uuid.UUID, secret []byte) (token string) {
	data := append(id.Bytes(), secret...)
	token = base64.StdEncoding.EncodeToString(data)
	return
}

func decodeSessionToken(ctx context.Context, token string) (sessionID uuid.UUID, secret []byte, err error) {
	var tokenBytes []byte

	tokenBytes, err = base64.StdEncoding.DecodeString(token)
	if err != nil {
		err = errors.Internal("users.decodeSessionToken: base64.StdEncoding.DecodeString(token)", err)
		return
	}

	if len(tokenBytes) != uuid.Size+crypto.KeySize512 {
		err = users.ErrInvalidSession
		return
	}

	sessionIDBytes := tokenBytes[:uuid.Size]
	secret = tokenBytes[uuid.Size:]

	sessionID, err = uuid.FromBytes(sessionIDBytes)
	if err != nil {
		err = users.ErrInvalidSession
		return
	}

	return
}

func formatCodeHyphen(code string) string {
	middle := len(code) / 2
	isEven := len(code)%2 == 0 && len(code) != 0

	if !isEven {
		return code
	}
	return code[:middle] + "-" + code[middle:]
}

func cleanCodeHyphen(code string) string {
	ret := ""
	for _, character := range code {
		if character != '-' {
			ret += string(character)
		}
	}
	return ret
}

// confirmationCodeToHTML transforms a confirmation code to an HTML span, with letters in red and
// symbols in blue for better readability
func confirmationCodeToHTML(code string) string {
	htmlCode := "<span>"

	for _, character := range code {
		if unicode.IsLetter(character) || character == '-' {
			htmlCode += string(character)
		} else if unicode.IsNumber(character) {
			htmlCode += `<span style="color: red">` + string(character) + `</span>`
		} else {
			htmlCode += `<span style="color: blue">` + string(character) + `</span>`
		}
	}

	htmlCode += "</span>"

	return htmlCode
}
