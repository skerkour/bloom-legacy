package users

import (
	"gitlab.com/bloom42/gobox/crypto"
)

const (
	PREFERENCES_KEY_MASTER_KEY  = "master_key"
	PREFERENCES_KEY_PUBLIC_KEY  = "public_key"
	PREFERENCES_KEY_PRIVATE_KEY = "private_key"

	// ME_KEY is the preferences key used to save me
	PREFERENCES_KEY_ME = "me"
	// SESSION_KEY is the preferences key used to save the session
	PREFERENCES_KEY_SESSION = "session"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
