package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

const (
	MASTER_KEY_KEY  = "master_key"
	PUBLIC_KEY_KEY  = "public_key"
	PRIVATE_KEY_KEY = "private_key"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
