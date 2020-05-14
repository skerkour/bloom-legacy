package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

const (
	MASTER_KEY_KEY  = "master_key"
	PUBLIC_KEY_KEY  = "public_key"
	PRIVATE_KEY_KEY = "private_key"

	// ME_KEY is the preferences key used to save me
	ME_KEY = "me"
	// SESSION_KEY is the preferences key used to save the session
	SESSION_KEY = "session"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
