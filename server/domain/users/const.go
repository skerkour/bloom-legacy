package users

import "gitlab.com/bloom42/gobox/crypto"

const (
	VerificationCodeAlphabet = "abcdefghijkmnpqrstuvwxyz23456789#@!"
	// RegisterCodeLength is the length in characters of the verification code sent for registration
	RegisterCodeLength = 10

	BioMaxLength         = 256
	DisplayNameMaxLength = 42
	FirstNameMaxLength   = 30
	LastNameMaxLength    = 42

	UsernameMinLength = 5
	UsernameMaxLength = 16

	AvatarMaxSize    = 3000000
	AvatarResize     = 256
	AvatarDefaultURL = "/imgs/myaccount/profile.jpg"

	// RegistrationMaxFailedAttempts is the max number of invalid code an user can try to validate its account with
	RegistrationMaxFailedAttempts = 5
	// SignInMaxFailedAttempts is the max number of invalid code an user can try to validate its session with
	SignInMaxFailedAttempts = 5

	SessionSecretSize = crypto.KeySize512
)

var PendingUserCodeHashParams = crypto.DefaultHashPasswordParams
var PasswordUpdateCodehashParams = PendingUserCodeHashParams
var AuthKeyHashParams = &crypto.HashPasswordParams{
	Memory:      6 * 1024,
	Iterations:  1,
	Parallelism: 1,
	SaltLength:  crypto.KeySize512,
	KeyLength:   crypto.KeySize512,
}

var InvalidUsername = []string{
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
