package keys

import (
	"gitlab.com/bloom42/gobox/crypto"
)

const (
	PREFERENCES_KEY_MASTER_KEY  = "master_key"
	PREFERENCES_KEY_PUBLIC_KEY  = "public_key"
	PREFERENCES_KEY_PRIVATE_KEY = "private_key"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
