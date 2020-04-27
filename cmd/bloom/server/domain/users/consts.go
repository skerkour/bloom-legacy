package users

import "gitlab.com/bloom42/lily/crypto"

const (
	USER_VERIFICATION_CODE_ALPHABET = "abcdefghijklmopqrstuvwxyz0123456789"

	BIO_MAX_LENGTH          = 200
	DISPLAY_NAME_MAX_LENGTH = 42
	FIRST_NAME_MAX_LENGTH   = 30
	LAST_NAME_MAX_LENGTH    = 42

	USERNAME_MIN_LENGTH = 5
	USERNAME_MAX_LENGTH = 16

	AVATAR_BYTES_MAX           = 3000000
	AVATAR_RESIZE              = 256
	PASSWORD_RESET_TOKEN_BYTES = 256
	AVATAR_DEFAULT_PATH        = "/imgs/myaccount/profile.jpg"

	MAX_REGISTRATION_ATTEMPTS = 5
)

var PENDING_USER_CODE_HASH_PARAMS = crypto.DefaultHashPasswordParams
var UPDATE_PASSWORD_CODE_HASH_PARAMS = PENDING_USER_CODE_HASH_PARAMS
var AUTH_KEY_HASH_PARAMS = PENDING_USER_CODE_HASH_PARAMS

var INVALID_USERNAMES = []string{
	"admin",
	"sysy",
	"administrator",
	"bloom",
	"bloom42",
	"support",
	"help",
	"settings",
	"security",
	"profile",
	"42bloom",
	"sylvain.kerkour",
	"sylvainkerkour",
	"kerkour.sylvain",
	"kerkoursylvain",
	"hello",
}
