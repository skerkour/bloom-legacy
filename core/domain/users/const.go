package users

import (
	"gitlab.com/bloom42/gobox/crypto"
)

const (
	// ME_KEY is the preferences key used to save me
	PREFERENCES_KEY_ME = "me"
	// SESSION_KEY is the preferences key used to save the session
	PREFERENCES_KEY_SESSION = "session"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
