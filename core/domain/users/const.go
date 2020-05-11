package users

import (
	"gitlab.com/bloom42/lily/crypto"
)

const (
	MASTER_KEY_KEY = "master_key"
)

var PASSWORD_KDF_PARAMS = crypto.DefaultDeriveKeyFromPasswordParams
