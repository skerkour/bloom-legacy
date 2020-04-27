package users

import "gitlab.com/bloom42/lily/crypto"

const (
	USER_VERIFICATION_CODE_ALPHABET = "abcdefghijklmopqrstuvwxyz0123456789"
)

var PENDING_USER_CODE_HASH_PARAMS = crypto.DefaultHashPasswordParams
var UPDATE_PASSWORD_CODE_HASH_PARAMS = PENDING_USER_CODE_HASH_PARAMS
var AUTH_KEY_HASH_PARAMS = PENDING_USER_CODE_HASH_PARAMS
